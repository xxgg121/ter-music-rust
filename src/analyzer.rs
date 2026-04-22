// 音频分析器模块 - 用于实时捕获音频数据并计算音量级别

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use rodio::source::SeekError;
use rodio::Source;

/// 音频分析器包装器，用于捕获音频样本并计算 RMS 音量
/// 专门处理 f32 格式的音频样本
pub struct AudioAnalyzer {
    /// 内部音频源（已转换为 f32）
    source: Box<dyn Source<Item = f32> + Send>,
    /// 当前音量级别 (0-1000，代表 0.0-1.0 的音量)
    volume_level: Arc<AtomicU32>,
    /// 样本平方和（用于计算 RMS）
    sum_of_squares: f64,
    /// 样本计数
    sample_count: usize,
    /// 每次计算 RMS 的样本数（约 30ms 的音频）
    samples_per_update: usize,
    /// EMA 平滑后的音量值 (0.0-1.0)
    smoothed_level: f64,
    /// EMA 平滑系数 (0.0-1.0)，值越小越平滑
    smooth_alpha: f64,
}

impl AudioAnalyzer {
    /// 创建新的音频分析器
    pub fn new(source: Box<dyn Source<Item = f32> + Send>, volume_level: Arc<AtomicU32>) -> Self {
        // 根据采样率计算约 30ms 的样本数
        let sample_rate = source.sample_rate();
        let samples_per_update = ((sample_rate as f64 * 0.03).max(100.0)) as usize;

        AudioAnalyzer {
            source,
            volume_level,
            sum_of_squares: 0.0,
            sample_count: 0,
            samples_per_update,
            smoothed_level: 0.0,
            smooth_alpha: 0.3, // EMA 系数：0.3 提供较好的平滑效果，同时保持响应速度
        }
    }
    
    /// 计算 RMS（均方根）音量，并使用 EMA 平滑
    fn calculate_rms(&mut self) {
        if self.sample_count == 0 {
            return;
        }
        
        // 计算 RMS
        let mean_square = self.sum_of_squares / self.sample_count as f64;
        let rms = mean_square.sqrt();
        
        // 将 RMS 转换为 0.0-1.0 的归一化值
        // 使用非线性映射使小音量更明显
        let normalized = (rms * 3.5).min(1.0); // 放大小音量
        
        // EMA 平滑：smoothed = alpha * new + (1 - alpha) * old
        // 平滑衰减：音量下降时使用更小的 alpha，避免波形突然消失
        let alpha = if normalized < self.smoothed_level {
            self.smooth_alpha // 下降时更慢，视觉更柔和
        } else {
            self.smooth_alpha // 上升时保持正常响应速度
        };
        self.smoothed_level = alpha * normalized + (1.0 - alpha) * self.smoothed_level;
        
        // 转换为 0-1000 的整数
        let level = (self.smoothed_level * 1000.0) as u32;
        
        self.volume_level.store(level, Ordering::Relaxed);
        
        // 重置计数器
        self.sum_of_squares = 0.0;
        self.sample_count = 0;
    }
}

impl Iterator for AudioAnalyzer {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.source.next()?;

        // 累加样本平方
        let sample_f64 = sample as f64;
        self.sum_of_squares += sample_f64 * sample_f64;
        self.sample_count += 1;

        // 每处理约 30ms 的样本后计算 RMS
        if self.sample_count >= self.samples_per_update {
            self.calculate_rms();
        }

        Some(sample)
    }
}

impl Source for AudioAnalyzer {
    fn current_frame_len(&self) -> Option<usize> {
        self.source.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.source.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.source.total_duration()
    }

    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        self.source.try_seek(pos)
    }
}
