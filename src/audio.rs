// 音频播放模块

use std::io::BufReader;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

use crate::analyzer::AudioAnalyzer;
use crate::defs::{MusicFile, PlayMode, PlayState};

/// 音频播放器
pub struct AudioPlayer {
    /// 音频输出流（必须保持存活）
    _stream: Option<OutputStream>,
    /// 音频流句柄
    _stream_handle: Option<OutputStreamHandle>,
    /// 播放控制器
    sink: Option<Sink>,

    /// 当前播放状态
    state: PlayState,
    /// 当前播放文件
    current_file: Option<MusicFile>,
    /// 当前音量（0-100）
    volume: Arc<AtomicU8>,
    /// 是否播放完成
    finished: Arc<AtomicBool>,
    /// 播放模式
    play_mode: PlayMode,
    
    /// 实时音量级别 (0-1000，代表 0.0-1.0 的音量)
    volume_level: Arc<AtomicU32>,

    /// 播放开始时间
    play_start_time: Option<Instant>,
    /// 累计播放时间（用于暂停恢复）
    accumulated_time: Duration,
    /// 歌曲总时长
    total_duration: Option<Duration>,
}

impl AudioPlayer {
    /// 创建新的音频播放器
    pub fn new() -> Self {
        AudioPlayer {
            _stream: None,
            _stream_handle: None,
            sink: None,
            state: PlayState::Stopped,
            current_file: None,
            volume: Arc::new(AtomicU8::new(50)),
            finished: Arc::new(AtomicBool::new(false)),
            play_mode: PlayMode::default(),
            volume_level: Arc::new(AtomicU32::new(0)),
            play_start_time: None,
            accumulated_time: Duration::ZERO,
            total_duration: None,
        }
    }

    /// 播放音乐文件
    pub fn play(&mut self, file: &MusicFile) -> Result<(), String> {
        // 停止当前播放
        self.stop();

        // 初始化音频输出流
        let (stream, stream_handle) =
            OutputStream::try_default().map_err(|e| format!("无法初始化音频输出: {}", e))?;

        // 创建播放控制器
        let sink =
            Sink::try_new(&stream_handle).map_err(|e| format!("无法创建播放控制器: {}", e))?;

        // 打开音频文件
        let audio_file = std::fs::File::open(&file.path)
            .map_err(|e| format!("无法打开音频文件 {:?}: {}", file.path, e))?;

        // 创建音频解码器
        let source = Decoder::new(BufReader::new(audio_file))
            .map_err(|e| format!("无法解码音频文件: {}", e))?;

        // 获取总时长
        self.total_duration = source.total_duration();
        
        // 重置音量级别
        self.volume_level.store(0, Ordering::Relaxed);

        // 将音频源转换为 f32 格式，然后创建音频分析器
        let source_f32: Box<dyn Source<Item = f32> + Send> = 
            Box::new(source.convert_samples());
        let analyzed_source = AudioAnalyzer::new(source_f32, self.volume_level.clone());

        // 设置音量
        let vol = self.volume.load(Ordering::SeqCst) as f32 / 100.0;
        sink.set_volume(vol);

        // 开始播放（使用分析器包装的源）
        sink.append(analyzed_source);

        // 保存状态
        self._stream = Some(stream);
        self._stream_handle = Some(stream_handle);
        self.sink = Some(sink);
        self.state = PlayState::Playing;
        self.current_file = Some(file.clone());
        self.finished.store(false, Ordering::SeqCst);

        // 重置时间追踪
        self.play_start_time = Some(Instant::now());
        self.accumulated_time = Duration::ZERO;

        Ok(())
    }

    /// 暂停播放
    pub fn pause(&mut self) {
        if let Some(sink) = &self.sink {
            sink.pause();
            self.state = PlayState::Paused;

            // 记录已播放时间
            if let Some(start) = self.play_start_time {
                self.accumulated_time += start.elapsed();
                self.play_start_time = None;
            }
        }
    }

    /// 继续播放
    pub fn resume(&mut self) {
        if let Some(sink) = &self.sink {
            sink.play();
            self.state = PlayState::Playing;

            // 重新开始计时
            self.play_start_time = Some(Instant::now());
        }
    }

    /// 停止播放
    pub fn stop(&mut self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }

        self.sink = None;
        self._stream_handle = None;
        self._stream = None;
        self.state = PlayState::Stopped;
        self.current_file = None;
        self.finished.store(false, Ordering::SeqCst);
        
        // 重置音量级别
        self.volume_level.store(0, Ordering::Relaxed);

        // 重置时间
        self.play_start_time = None;
        self.accumulated_time = Duration::ZERO;
        self.total_duration = None;
    }

    /// 获取当前播放进度 (已播放时间, 总时长)
    pub fn get_progress(&self) -> (Duration, Option<Duration>) {
        let current = match self.state {
            PlayState::Playing => {
                if let Some(start) = self.play_start_time {
                    self.accumulated_time + start.elapsed()
                } else {
                    self.accumulated_time
                }
            }
            PlayState::Paused => self.accumulated_time,
            PlayState::Stopped => Duration::ZERO,
        };

        (current, self.total_duration)
    }

    /// 格式化时间显示 (mm:ss / mm:ss)
    pub fn format_progress(&self) -> String {
        let (current, total) = self.get_progress();

        let current_secs = current.as_secs();
        let current_mins = current_secs / 60;
        let current_secs = current_secs % 60;

        if let Some(total) = total {
            let total_secs = total.as_secs();
            let total_mins = total_secs / 60;
            let total_secs = total_secs % 60;
            format!(
                "{:02}:{:02}/{:02}:{:02}",
                current_mins, current_secs, total_mins, total_secs
            )
        } else {
            format!("{:02}:{:02}/--:--", current_mins, current_secs)
        }
    }

    /// 设置音量（0-100）
    pub fn set_volume(&mut self, volume: u8) {
        let vol = volume.min(100);
        self.volume.store(vol, Ordering::SeqCst);

        if let Some(sink) = &self.sink {
            sink.set_volume(vol as f32 / 100.0);
        }
    }

    /// 获取音量
    pub fn get_volume(&self) -> u8 {
        self.volume.load(Ordering::SeqCst)
    }

    /// 音量增加
    pub fn volume_up(&mut self) {
        let vol = self.volume.load(Ordering::SeqCst);
        if vol < 100 {
            self.set_volume(vol + 5);
        }
    }

    /// 音量减少
    pub fn volume_down(&mut self) {
        let vol = self.volume.load(Ordering::SeqCst);
        if vol >= 5 {
            self.set_volume(vol - 5);
        } else {
            self.set_volume(0);
        }
    }

    /// 获取播放状态
    pub fn get_state(&self) -> PlayState {
        self.state
    }

    /// 获取当前播放文件
    pub fn get_current_file(&self) -> Option<MusicFile> {
        self.current_file.clone()
    }

    /// 检查是否播放完成
    pub fn is_finished(&self) -> bool {
        if let Some(sink) = &self.sink {
            sink.empty()
        } else {
            true
        }
    }

    /// 设置播放模式
    pub fn set_play_mode(&mut self, mode: PlayMode) {
        self.play_mode = mode;
    }

    /// 获取播放模式
    pub fn get_play_mode(&self) -> PlayMode {
        self.play_mode
    }
    
    /// 跳转到指定比例位置播放（0.0-1.0）
    pub fn seek(&mut self, ratio: f64) -> Result<(), String> {
        if let Some(sink) = &self.sink {
            if let Some(total) = self.total_duration {
                let target_time = Duration::from_secs_f64(total.as_secs_f64() * ratio.clamp(0.0, 1.0));
                sink.try_seek(target_time)
                    .map_err(|e| format!("跳转失败: {}", e))?;

                // 更新时间追踪
                self.accumulated_time = target_time;
                if self.state == PlayState::Playing {
                    self.play_start_time = Some(Instant::now());
                } else {
                    self.play_start_time = None;
                }

                Ok(())
            } else {
                Err("无法跳转：未知歌曲时长".to_string())
            }
        } else {
            Err("无法跳转：未在播放".to_string())
        }
    }

    /// 获取实时音量级别 (0.0-1.0)
    pub fn get_realtime_volume(&self) -> f32 {
        let level = self.volume_level.load(Ordering::Relaxed);
        level as f32 / 1000.0
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new()
    }
}
