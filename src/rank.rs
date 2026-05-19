pub struct PresetRank {
    pub name: &'static str,
    pub url: &'static str,
}

pub const PRESET_RANKS: &[PresetRank] = &[
    PresetRank {
        name: "每周热门全球",
        url: "https://music.apple.com/cn/playlist/%E6%AF%8F%E5%91%A8%E7%83%AD%E9%97%A8-100-%E9%A6%96-%E5%85%A8%E7%90%83/pl.921750b485a6496ea58b16d46c097557",
    },
    PresetRank {
        name: "每周热门大陆",
        url: "https://music.apple.com/cn/playlist/%E6%AF%8F%E5%91%A8%E7%83%AD%E9%97%A8-100-%E9%A6%96-%E4%B8%AD%E5%9B%BD%E5%A4%A7%E9%99%86/pl.939cf56e73c44970b81fd9648f859223",
    },
    PresetRank {
        name: "每周热门香港",
        url: "https://music.apple.com/cn/playlist/%E6%AF%8F%E5%91%A8%E7%83%AD%E9%97%A8-100-%E9%A6%96-%E4%B8%AD%E5%9B%BD%E9%A6%99%E6%B8%AF/pl.f600030d19174703ab6e37605a6bec08",
    },
    PresetRank {
        name: "每周热门台湾",
        url: "https://music.apple.com/tw/playlist/top-100-%E5%8F%B0%E7%81%A3/pl.741ff34016704547853b953ec5181d83",
    },
    PresetRank {
        name: "每周热门美国",
        url: "https://music.apple.com/cn/playlist/%E6%AF%8F%E5%91%A8%E7%83%AD%E9%97%A8-100-%E9%A6%96-%E7%BE%8E%E5%9B%BD/pl.6f4d1d4d6eae48579cead6a7bc2a0c0d",
    },
    PresetRank {
        name: "每周热门韩国",
        url: "https://music.apple.com/cn/playlist/%E6%AF%8F%E5%91%A8%E7%83%AD%E9%97%A8-100-%E9%A6%96-%E9%9F%A9%E5%9B%BD/pl.4a5c566712634cb1914ec3d104a9e4db",
    },
    PresetRank {
        name: "每周热门日本",
        url: "https://music.apple.com/cn/playlist/%E6%AF%8F%E5%91%A8%E7%83%AD%E9%97%A8-100-%E9%A6%96-%E6%97%A5%E6%9C%AC/pl.417f0970ea794ee9b7c819f6d2324821",
    },
    PresetRank {
        name: "每周热门英国",
        url: "https://music.apple.com/cn/playlist/%E6%AF%8F%E5%91%A8%E7%83%AD%E9%97%A8-100-%E9%A6%96-%E8%8B%B1%E5%9B%BD/pl.23a3552e92134bbbb8bc7acbc27c4e5a",
    },
    PresetRank {
        name: "每周热门澳州",
        url: "https://music.apple.com/cn/playlist/%E6%AF%8F%E5%91%A8%E7%83%AD%E9%97%A8-100-%E9%A6%96-%E6%BE%B3%E5%A4%A7%E5%88%A9%E4%BA%9A/pl.41b58acae5f2447ebb0047c8a62ad8d6",
    },
    PresetRank {
        name: "百大榜：全球",
        url: "https://music.apple.com/hk/playlist/%E7%99%BE%E5%A4%A7%E6%A6%9C-%E5%85%A8%E7%90%83/pl.d25f5d1181894928af76c85c967f8f31",
    },
    PresetRank {
        name: "百大榜：香港",
        url: "https://music.apple.com/hk/playlist/%E7%99%BE%E5%A4%A7%E6%A6%9C-%E9%A6%99%E6%B8%AF/pl.7f35cffa10b54b91aab128ccc547f6ef",
    },
    PresetRank {
        name: "百大榜：美国",
        url: "https://music.apple.com/hk/playlist/%E7%99%BE%E5%A4%A7%E6%A6%9C-%E7%BE%8E%E5%9C%8B/pl.606afcbb70264d2eb2b51d8dbcfa6a12",
    },
    PresetRank {
        name: "百大榜：韩国",
        url: "https://music.apple.com/hk/playlist/%E7%99%BE%E5%A4%A7%E6%A6%9C-%E5%8D%97%E9%9F%93/pl.d3d10c32fbc540b38e266367dc8cb00c",
    },
    PresetRank {
        name: "百大榜：日本",
        url: "https://music.apple.com/hk/playlist/%E7%99%BE%E5%A4%A7%E6%A6%9C-%E6%97%A5%E6%9C%AC/pl.043a2c9876114d95a4659988497567be",
    },
    PresetRank {
        name: "百大榜：英国",
        url: "https://music.apple.com/hk/playlist/%E7%99%BE%E5%A4%A7%E6%A6%9C-%E8%8B%B1%E5%9C%8B/pl.c2273b7e89b44121b3093f67228918e7",
    },
    PresetRank {
        name: "百大榜：澳洲",
        url: "https://music.apple.com/hk/playlist/%E7%99%BE%E5%A4%A7%E6%A6%9C-%E6%BE%B3%E6%B4%B2/pl.18be1cf04dfd4ffb9b6b0453e8fae8f1",
    },
    PresetRank {
        name: "百大榜：法国",
        url: "https://music.apple.com/hk/playlist/%E7%99%BE%E5%A4%A7%E6%A6%9C-%E6%B3%95%E5%9C%8B/pl.6e8cfd81d51042648fa36c9df5236b8d",
    },
    PresetRank {
        name: "百大榜：德国",
        url: "https://music.apple.com/hk/playlist/%E7%99%BE%E5%A4%A7%E6%A6%9C-%E5%BE%B7%E5%9C%8B/pl.c10a2c113db14685a0b09fa5834d8e8b",
    },
    PresetRank {
        name: "大陆最新歌曲",
        url: "https://music.apple.com/cn/room/6769587587",
    },
    PresetRank {
        name: "香港必听新歌",
        url: "https://music.apple.com/hk/room/6769458778",
    },
    PresetRank {
        name: "台湾新歌精选",
        url: "https://music.apple.com/tw/room/6769458699",
    },
    PresetRank {
        name: "大陆正在流行",
        url: "https://music.apple.com/cn/room/6769587728",
    },
    PresetRank {
        name: "香港正在流行",
        url: "https://music.apple.com/hk/room/6761185183",
    },
    PresetRank {
        name: "台湾正在流行",
        url: "https://music.apple.com/tw/room/6769458811",
    },
    PresetRank {
        name: "大陆热门排行",
        url: "https://music.apple.com/cn/new/top-charts/songs",
    },
    PresetRank {
        name: "香港热门排行",
        url: "https://music.apple.com/hk/new/top-charts/songs",
    },
    PresetRank {
        name: "台湾热门排行",
        url: "https://music.apple.com/tw/new/top-charts/songs",
    },
    PresetRank {
        name: "美国热门排行",
        url: "https://music.apple.com/us/new/top-charts/songs",
    },
    PresetRank {
        name: "韩国热门排行",
        url: "https://music.apple.com/kr/new/top-charts/songs",
    },
    PresetRank {
        name: "日本热门排行",
        url: "https://music.apple.com/jp/new/top-charts/songs",
    },
    PresetRank {
        name: "最新热门歌曲",
        url: "https://music.apple.com/cn/room/6503392868",
    },
    PresetRank {
        name: "城市排行北京",
        url: "https://music.apple.com/cn/playlist/top-25-%E5%8C%97%E4%BA%AC/pl.85439c5739b547c6a805a0aede6a7865",
    },
    PresetRank {
        name: "城市排行上海",
        url: "https://music.apple.com/cn/playlist/top-25-%E4%B8%8A%E6%B5%B7/pl.4cc86bb8172b45f4a2f4aec473176320",
    },
    PresetRank {
        name: "城市排行广州",
        url: "https://music.apple.com/cn/playlist/top-25-%E5%B9%BF%E5%B7%9E/pl.5b45c8decd4c4c40b40239136fe5b9ff",
    },
    PresetRank {
        name: "城市排行成都",
        url: "https://music.apple.com/cn/playlist/top-25-%E6%88%90%E9%83%BD/pl.a91e9af475e447fcb7252f6e0a5aa72e",
    },
    PresetRank {
        name: "城市排行武汉",
        url: "https://music.apple.com/cn/playlist/top-25-%E6%AD%A6%E6%B1%89/pl.1812e61fc0d940dd988461238cc8a6b2",
    },
    PresetRank {
        name: "热播国语流行",
        url: "https://music.apple.com/cn/playlist/%E7%83%AD%E6%92%AD%E9%87%91%E6%9B%B2-%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C/pl.6d8228f57b864a4296dc02d9761a0d9b",
    },
    PresetRank {
        name: "热播粤语流行",
        url: "https://music.apple.com/cn/playlist/%E7%83%AD%E6%92%AD%E9%87%91%E6%9B%B2-%E7%B2%A4%E8%AF%AD%E6%B5%81%E8%A1%8C/pl.ad09ef940a6e4fbe945eafab033d1aaf",
    },
    PresetRank {
        name: "全球热门歌曲",
        url: "https://open.spotify.com/playlist/37i9dQZEVXbNG2KDcFcKOF",
    },
    PresetRank {
        name: "香港热门歌曲",
        url: "https://open.spotify.com/playlist/37i9dQZEVXbMdvweCgpBAe",
    },
    PresetRank {
        name: "全球前 50 名",
        url: "https://open.spotify.com/playlist/37i9dQZEVXbMDoHDwVN2tF",
    },
    PresetRank {
        name: "香港前 50 名",
        url: "https://open.spotify.com/playlist/37i9dQZEVXbLwpL8TjsxOG",
    },

    PresetRank {
        name: "香港人气劲歌",
        url: "https://music.apple.com/hk/playlist/%E4%BA%BA%E6%B0%A3%E5%8B%81%E6%AD%8C/pl.f4d106fed2bd41149aaacabb233eb5eb",
    },
    PresetRank {
        name: "香港 Hip-Hop",
        url: "https://music.apple.com/cn/playlist/%E9%A6%99%E6%B8%AF-hip-hop-%E4%BB%A3%E8%A1%A8%E4%BD%9C%E5%93%81/pl.c95eddfa740a447b8851f017e4c8bc58",
    },
    PresetRank {
        name: "1980年代流行",
        url: "https://music.apple.com/cn/playlist/80-%E5%B9%B4%E4%BB%A3%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E4%BB%A3%E8%A1%A8%E4%BD%9C%E5%93%81/pl.9601a5e6e3d44f6eba2d8ebff1903610",
    },
    PresetRank {
        name: "1980年代情歌",
        url: "https://music.apple.com/cn/playlist/80s-mandopop-love-songs/pl.8bcc62c87ed3496ea55bde3a44711527",
    },
    PresetRank {
        name: "1990年代流行",
        url: "https://music.apple.com/cn/playlist/90-%E5%B9%B4%E4%BB%A3%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E4%BB%A3%E8%A1%A8%E4%BD%9C%E5%93%81/pl.9c71913d52fe4c3eb7069ac661ab46e6",
    },
    PresetRank {
        name: "1990年代情歌",
        url: "https://music.apple.com/cn/playlist/90-%E5%B9%B4%E4%BB%A3%E5%9B%BD%E8%AF%AD%E6%83%85%E6%AD%8C%E4%BB%A3%E8%A1%A8%E4%BD%9C%E5%93%81/pl.5ed96d329c8d4d2097affb00b9d0383d",
    },
    
    PresetRank {
        name: "2000年代流行",
        url: "https://music.apple.com/cn/playlist/2000-%E5%B9%B4%E4%BB%A3%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E4%BB%A3%E8%A1%A8%E4%BD%9C%E5%93%81/pl.b5c94ec72f3041d6844ad0772a36f001",
    },
    PresetRank {
        name: "2000年代情歌",
        url: "https://music.apple.com/cn/playlist/2000-%E5%B9%B4%E4%BB%A3%E5%9B%BD%E8%AF%AD%E6%83%85%E6%AD%8C%E4%BB%A3%E8%A1%A8%E4%BD%9C%E5%93%81/pl.6cef5e2b3485456287e6e954a69dea8b",
    },
    PresetRank {
        name: "2010年代流行",
        url: "https://music.apple.com/cn/playlist/2010-%E5%B9%B4%E4%BB%A3%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E4%BB%A3%E8%A1%A8%E4%BD%9C%E5%93%81/pl.3e52664b20ea45c394a37f4a7f3a8451",
    },
    PresetRank {
        name: "2010年代情歌",
        url: "https://music.apple.com/cn/playlist/2010-%E5%B9%B4%E4%BB%A3%E5%9B%BD%E8%AF%AD%E6%83%85%E6%AD%8C%E4%BB%A3%E8%A1%A8%E4%BD%9C%E5%93%81/pl.5ffa6ae5975e4ce5be34ea6bc39854c5",
    },
    PresetRank {
        name: "国语流行1990",
        url: "https://music.apple.com/cn/playlist/1990-%E5%B9%B4%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%95%85%E9%94%80%E9%87%91%E6%9B%B2/pl.a5e8da9d7afe4148b4cb23c2f156f53c",
    },
    PresetRank {
        name: "国语流行1995",
        url: "https://music.apple.com/cn/playlist/1995-%E5%B9%B4%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%95%85%E9%94%80%E9%87%91%E6%9B%B2/pl.41339d834fa04e0d8ff1238672f056a9",
    },
    PresetRank {
        name: "国语流行1996",
        url: "https://music.apple.com/cn/playlist/1996-%E5%B9%B4%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%95%85%E9%94%80%E9%87%91%E6%9B%B2/pl.9bc758af94a84da8bcf81dc2e4071617",
    },
    PresetRank {
        name: "国语流行1997",
        url: "https://music.apple.com/cn/playlist/1997-%E5%B9%B4%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%95%85%E9%94%80%E9%87%91%E6%9B%B2/pl.78c647d7de7349c7bd6b314a547776d5",
    },
    PresetRank {
        name: "国语流行1998",
        url: "https://music.apple.com/cn/playlist/1998-%E5%B9%B4%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%95%85%E9%94%80%E9%87%91%E6%9B%B2/pl.fb867f8172cc4da5a1cd24e0d84eeb37",
    },
    PresetRank {
        name: "国语流行1999",
        url: "https://music.apple.com/cn/playlist/1999-%E5%B9%B4%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%95%85%E9%94%80%E9%87%91%E6%9B%B2/pl.d3d6242e1f1d4410bddf3fc013a6e95f",
    },
    PresetRank {
        name: "国语流行2000",
        url: "https://music.apple.com/cn/playlist/2000-%E5%B9%B4%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%95%85%E9%94%80%E9%87%91%E6%9B%B2/pl.8a2e161509744600ba82d7d2d2c64b53",
    },
    PresetRank {
        name: "国语流行2001",
        url: "https://music.apple.com/cn/playlist/2001-%E5%B9%B4%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%95%85%E9%94%80%E9%87%91%E6%9B%B2/pl.8b52c25fb5b84910afbd7e5e7957b393",
    },
    PresetRank {
        name: "国语流行2002",
        url: "https://music.apple.com/cn/playlist/2002-%E5%B9%B4%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%95%85%E9%94%80%E9%87%91%E6%9B%B2/pl.aed839967602471e8e8de0063671ba3d",
    },
    PresetRank {
        name: "国语流行2003",
        url: "https://music.apple.com/cn/playlist/2003-%E5%B9%B4%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%95%85%E9%94%80%E9%87%91%E6%9B%B2/pl.1d022466efe941a787b6a8b1a4d99f9b",
    },
    PresetRank {
        name: "国语流行2004",
        url: "https://music.apple.com/cn/playlist/2004-%E5%B9%B4%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%95%85%E9%94%80%E9%87%91%E6%9B%B2/pl.5b592259afcf480f99ad30f6d25243d5",
    },
    PresetRank {
        name: "国语流行2005",
        url: "https://music.apple.com/cn/playlist/2005-%E5%B9%B4%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%95%85%E9%94%80%E9%87%91%E6%9B%B2/pl.1fae9425c12b45bb899ef13937a29290",
    },
    PresetRank {
        name: "国语流行2006",
        url: "https://music.apple.com/cn/playlist/2006-%E5%B9%B4%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%95%85%E9%94%80%E9%87%91%E6%9B%B2/pl.e231b9cf7b03478f8d5a1f99ebb02a20",
    },
    PresetRank {
        name: "国语流行2007",
        url: "https://music.apple.com/cn/playlist/2007-%E5%B9%B4%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%95%85%E9%94%80%E9%87%91%E6%9B%B2/pl.cf35e8031da84ba6a1c34f1f8c0d0666",
    },
    PresetRank {
        name: "国语流行2008",
        url: "https://music.apple.com/cn/playlist/2008-%E5%B9%B4%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%95%85%E9%94%80%E9%87%91%E6%9B%B2/pl.cc8fb0f0a2aa4e289107b31a64296975",
    },
    PresetRank {
        name: "国语流行2009",
        url: "https://music.apple.com/cn/playlist/2009-%E5%B9%B4%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%95%85%E9%94%80%E9%87%91%E6%9B%B2/pl.55294a58615e49809b072bcc528fc35c",
    },
    PresetRank {
        name: "国语流行2025",
        url: "https://music.apple.com/cn/playlist/%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C%E7%83%AD%E6%AD%8C-2025/pl.7e7a5f6fe95e4e6393bf85df3a5d5dc2",
    },

    PresetRank {
        name: "A-List年度最佳",
        url: "https://music.apple.com/cn/playlist/a-list-%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C-2025-%E5%B9%B4%E5%BA%A6%E6%9C%80%E4%BD%B3/pl.f1a4032f4b9548088e13944c6d2e2bfe",
    },
    PresetRank {
        name: "A-List国语流行",
        url: "https://music.apple.com/cn/playlist/a-list-%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C/pl.beb783da7712481fbeed35be144bd48c",
    },
    PresetRank {
        name: "A-List粤语流行",
        url: "https://music.apple.com/cn/playlist/a-list-%E7%B2%A4%E8%AF%AD%E6%B5%81%E8%A1%8C/pl.31991c6bcf0447ac9f16c07678a5a0a0",
    },
    PresetRank {
        name: "A-List最佳粤语",
        url: "https://music.apple.com/cn/playlist/a-list-%E7%B2%A4%E8%AF%AD%E6%B5%81%E8%A1%8C-2025-%E5%B9%B4%E5%BA%A6%E6%9C%80%E4%BD%B3/pl.cbea022ec1054554aaaefefb851b0b48",
    },
    PresetRank {
        name: "A-List华语流行",
        url: "https://music.apple.com/tw/playlist/a-list-%E8%8F%AF%E8%AA%9E%E6%B5%81%E8%A1%8C%E6%A8%82/pl.beb783da7712481fbeed35be144bd48c",
    },
    PresetRank {
        name: "A-List国际流行",
        url: "https://music.apple.com/cn/playlist/a-list-%E5%9B%BD%E9%99%85%E6%B5%81%E8%A1%8C/pl.5ee8333dbe944d9f9151e97d92d1ead9",
    },
    PresetRank {
        name: "合家欢乐金曲",
        url: "https://music.apple.com/cn/playlist/%E5%90%88%E5%AE%B6%E6%AC%A2%E9%87%91%E6%9B%B2/pl.cd835923714f47f295c08470b18e8891",
    },
    PresetRank {
        name: "热播KPOPWRLD",
        url: "https://music.apple.com/cn/playlist/kpopwrld/pl.48229b41bbfc47d7af39dae8e8b5276e",
    },
    PresetRank {
        name: "国语情歌精选",
        url: "https://music.apple.com/cn/playlist/%E5%9B%BD%E8%AF%AD%E6%83%85%E6%AD%8C%E7%B2%BE%E9%80%89/pl.8e9831f8282c46bc80aceea9399bb32d",
    },
    PresetRank {
        name: "心碎国语情歌",
        url: "https://music.apple.com/cn/playlist/%E5%BF%83%E7%A2%8E%E5%9B%BD%E8%AF%AD%E6%83%85%E6%AD%8C/pl.13dc58ad0eda4543b2169ae0a4ef13f6",
    },
    PresetRank {
        name: "粤语情歌精选",
        url: "https://music.apple.com/cn/playlist/%E7%B2%A4%E8%AF%AD%E6%83%85%E6%AD%8C%E7%B2%BE%E9%80%89/pl.f3a6402280a1424dafd244a740d69a68",
    },
    PresetRank {
        name: "心碎粤语情歌",
        url: "https://music.apple.com/cn/playlist/%E5%BF%83%E7%A2%8E%E7%B2%A4%E8%AF%AD%E6%83%85%E6%AD%8C/pl.8e2b9bab0d814ecab98f5e7c74cd9b21",
    },
    PresetRank {
        name: "唱歌粤语流行",
        url: "https://music.apple.com/cn/playlist/%E5%94%B1%E6%AD%8C-%E7%B2%A4%E8%AF%AD%E6%B5%81%E8%A1%8C/pl.cb022217e9864fe69ae79831db2f31ce",
    },
    PresetRank {
        name: "2010粤语情歌",
        url: "https://music.apple.com/cn/playlist/2010-%E5%B9%B4%E4%BB%A3%E7%B2%A4%E8%AF%AD%E6%83%85%E6%AD%8C%E4%BB%A3%E8%A1%A8%E4%BD%9C%E5%93%81/pl.03fb2a50d46b4a08aa76939499b965de",
    },
    PresetRank {
        name: "有氧运动粤语",
        url: "https://music.apple.com/cn/playlist/%E6%9C%89%E6%B0%A7%E8%BF%90%E5%8A%A8%E6%AD%8C%E5%8D%95-%E7%B2%A4%E8%AF%AD%E6%B5%81%E8%A1%8C/pl.aae554a482ff4414a6fd3b9c0c64ddf4",
    },
    PresetRank {
        name: "黄伟文-粤语",
        url: "https://music.apple.com/cn/playlist/%E5%88%9B%E4%BD%9C%E4%BA%BA-%E9%BB%84%E4%BC%9F%E6%96%87/pl.2e4db1c20ef1414bba0b8c9ec00b31cf",
    },
    PresetRank {
        name: "国语潜力新歌",
        url: "https://music.apple.com/cn/playlist/%E6%BD%9C%E5%8A%9B%E6%96%B0%E6%AD%8C-%E5%9B%BD%E8%AF%AD%E6%B5%81%E8%A1%8C/pl.80e13199d5db46c7b519b25cf6e5816a",
    },
    PresetRank {
        name: "粤语潜力新歌",
        url: "https://music.apple.com/cn/playlist/%E6%BD%9C%E5%8A%9B%E6%96%B0%E6%AD%8C-%E7%B2%A4%E8%AF%AD%E6%B5%81%E8%A1%8C/pl.873e73a132734e14810b0511b57a068c",
    },
    PresetRank {
        name: "90年代国语情歌",
        url: "https://music.apple.com/cn/playlist/90-%E5%B9%B4%E4%BB%A3%E5%9B%BD%E8%AF%AD%E6%83%85%E6%AD%8C%E4%BB%A3%E8%A1%A8%E4%BD%9C%E5%93%81/pl.5ed96d329c8d4d2097affb00b9d0383d",
    },
    PresetRank {
        name: "电影中的歌",
        url: "https://music.apple.com/cn/playlist/%E7%94%B5%E5%BD%B1%E4%B8%AD%E7%9A%84%E6%AD%8C/pl.15c7b4c4d92f4f96831270c1e88747b4",
    },
    PresetRank {
        name: "剧集中的歌",
        url: "https://music.apple.com/cn/playlist/%E5%89%A7%E9%9B%86%E4%B8%AD%E7%9A%84%E6%AD%8C/pl.f0730d489e8346b79be1e667340208e8",
    },
    PresetRank {
        name: "派对国语歌",
        url: "https://music.apple.com/cn/playlist/%E6%B4%BE%E5%AF%B9%E5%9B%BD%E8%AF%AD%E6%AD%8C/pl.86c84778a8a6420e8fc35003ad2bc7ee",
    },
    PresetRank {
        name: "国语健身歌",
        url: "https://music.apple.com/cn/playlist/%E5%9B%BD%E8%AF%AD%E5%81%A5%E8%BA%AB%E6%AD%8C/pl.abcfcd364050431db617cfa27c2ebb75",
    },
    PresetRank {
        name: "中嘻合璧歌",
        url: "https://music.apple.com/cn/playlist/%E4%B8%AD%E5%98%BB%E5%90%88%E7%92%A7/pl.9eba8510029645e0a4bd3d7fba087a4d",
    },
    PresetRank {
        name: "卡拉永远 OK",
        url: "https://music.apple.com/cn/playlist/%E5%8D%A1%E6%8B%89%E6%B0%B8%E8%BF%9C-ok/pl.3c940066ea8d47adb2d317c492a981fe",
    },
    PresetRank {
        name: "R&B 进行时",
        url: "https://music.apple.com/cn/playlist/r-b-%E8%BF%9B%E8%A1%8C%E6%97%B6/pl.b7ae3e0a28e84c5c96c4284b6a6c70af",
    },
    PresetRank {
        name: "网络热播C-Pop",
        url: "https://music.apple.com/cn/playlist/%E7%BD%91%E7%BB%9C%E7%83%AD%E6%92%AD-c-pop/pl.2a0a202d08c3439e95d22a73126f4417",
    },
    PresetRank {
        name: "C-Pop新歌榜",
        url: "https://music.apple.com/cn/playlist/c-pop-%E6%96%B0%E6%AD%8C/pl.b3dcb52a0a8c48f2a867ae62f63671e0",
    },
    PresetRank {
        name: "另类破格之声",
        url: "https://music.apple.com/cn/playlist/%E7%A0%B4%E6%A0%BC%E4%B9%8B%E5%A3%B0/pl.b362446682f543a3bfb40305ebb69e99",
    },
    PresetRank {
        name: "台湾新声说唱",
        url: "https://music.apple.com/cn/playlist/%E5%8F%B0%E6%B9%BE%E6%96%B0%E5%A3%B0/pl.3a20c04e83ef44a7a1af31c40f4c6a21",
    },
    PresetRank {
        name: "来自台湾精选",
        url: "https://music.apple.com/cn/playlist/%E6%9D%A5%E8%87%AA%E5%8F%B0%E6%B9%BE/pl.61b9c7ec7aa846cb99ea8af2804b4d51",
    },
    PresetRank {
        name: "古装剧集金曲",
        url: "https://music.apple.com/cn/playlist/%E5%8F%A4%E8%A3%85%E5%89%A7%E9%87%91%E6%9B%B2/pl.cdfd8967bb444d58bc7bbc83fe11abfa",
    },
    PresetRank {
        name: "活力 K-Pop",
        url: "https://music.apple.com/cn/playlist/%E6%B4%BB%E5%8A%9B-k-pop/pl.d838905f50af4200a2ebbc614922dee9",
    },
    
    PresetRank {
        name: "酷我飙升榜",
        url: "https://www.kuwo.cn/rankList",
    },
    PresetRank {
        name: "酷狗飙升榜",
        url: "https://www.kugou.com/yy/rank/home/1-6666.html?from=rank",
    },
    PresetRank {
        name: "酷狗TOP500",
        url: "https://www.kugou.com/yy/rank/home/1-8888.html?from=rank",
    },
    PresetRank {
        name: "蜂鸟流行榜",
        url: "https://www.kugou.com/yy/rank/home/1-59703.html?from=rank",
    },
    PresetRank {
        name: "抖音热歌榜",
        url: "https://www.kugou.com/yy/rank/home/1-52144.html?from=rank",
    },
    PresetRank {
        name: "快手热歌榜",
        url: "https://www.kugou.com/yy/rank/home/1-52767.html?from=rank",
    },
    PresetRank {
        name: "DJ热歌榜",
        url: "https://www.kugou.com/yy/rank/home/1-24971.html?from=rank",
    },
    PresetRank {
        name: "大陆流行榜",
        url: "https://www.kugou.com/yy/rank/home/1-31308.html?from=rank",
    },
    PresetRank {
        name: "香港流行榜",
        url: "https://www.kugou.com/yy/rank/home/1-31313.html?from=rank",
    },
    PresetRank {
        name: "台湾流行榜",
        url: "https://www.kugou.com/yy/rank/home/1-54848.html?from=rank",
    },
    PresetRank {
        name: "欧美流行榜",
        url: "https://www.kugou.com/yy/rank/home/1-31310.html?from=rank",
    },
    PresetRank {
        name: "韩国流行榜",
        url: "https://www.kugou.com/yy/rank/home/1-31311.html?from=rank",
    },
    PresetRank {
        name: "日本流行榜",
        url: "https://www.kugou.com/yy/rank/home/1-31312.html?from=rank",
    },
    PresetRank {
        name: "ACG新歌榜",
        url: "https://www.kugou.com/yy/rank/home/1-33162.html?from=rank",
    },
    PresetRank {
        name: "电音热歌榜",
        url: "https://www.kugou.com/yy/rank/home/1-33160.html?from=rank",
    },
    PresetRank {
        name: "综艺新歌榜",
        url: "https://www.kugou.com/yy/rank/home/1-46910.html?from=rank",
    },
    PresetRank {
        name: "说唱先锋榜",
        url: "https://www.kugou.com/yy/rank/home/1-44412.html?from=rank",
    },
    PresetRank {
        name: "影视金曲榜",
        url: "https://www.kugou.com/yy/rank/home/1-33163.html?from=rank",
    },
    PresetRank {
        name: "粤语金曲榜",
        url: "https://www.kugou.com/yy/rank/home/1-33165.html?from=rank",
    },
    PresetRank {
        name: "欧美金曲榜",
        url: "https://www.kugou.com/yy/rank/home/1-33166.html?from=rank",
    },
    PresetRank {
        name: "酷狗原创榜",
        url: "https://www.kugou.com/yy/rank/home/1-30972.html?from=rank",
    },
    PresetRank {
        name: "酷狗识曲榜",
        url: "https://www.kugou.com/yy/rank/home/1-37361.html?from=rank",
    },
    PresetRank {
        name: "80后热歌榜",
        url: "https://www.kugou.com/yy/rank/home/1-49225.html?from=rank",
    },
    PresetRank {
        name: "90后热歌榜",
        url: "https://www.kugou.com/yy/rank/home/1-49223.html?from=rank",
    },
    PresetRank {
        name: "00后热歌榜",
        url: "https://www.kugou.com/yy/rank/home/1-49224.html?from=rank",
    },
    PresetRank {
        name: "美国Bill榜",
        url: "https://www.kugou.com/yy/rank/home/1-4681.html?from=rank",
    },
    PresetRank {
        name: "英国单曲榜",
        url: "https://www.kugou.com/yy/rank/home/1-4680.html?from=rank",
    },
    PresetRank {
        name: "日本公信榜",
        url: "https://www.kugou.com/yy/rank/home/1-4673.html?from=rank",
    },
    PresetRank {
        name: "韩国音乐榜",
        url: "https://www.kugou.com/yy/rank/home/1-38623.html?from=rank",
    },
    PresetRank {
        name: "本地热歌榜",
        url: "https://www.kugou.com/yy/rank/home/1-42807.html?from=rank",
    },
    PresetRank {
        name: "KKBOX风云榜",
        url: "https://www.kugou.com/yy/rank/home/1-42808.html?from=rank",
    },
    PresetRank {
        name: "日本SPACE榜",
        url: "https://www.kugou.com/yy/rank/home/1-46868.html?from=rank",
    },
    PresetRank {
        name: "电子舞曲榜",
        url: "https://www.kugou.com/yy/rank/home/1-25028.html?from=rank",
    },
    PresetRank {
        name: "小语种热歌榜",
        url: "https://www.kugou.com/yy/rank/home/1-36107.html?from=rank",
    },
    PresetRank {
        name: "网易飙升榜",
        url: "https://music.163.com/#/discover/toplist?id=19723756",
    },
    PresetRank {
        name: "网易新歌榜",
        url: "https://music.163.com/#/discover/toplist?id=3779629",
    },
    PresetRank {
        name: "网易原创榜",
        url: "https://music.163.com/#/discover/toplist?id=2884035",
    },
    PresetRank {
        name: "网易热歌榜",
        url: "https://music.163.com/#/discover/toplist?id=3778678",
    },
    PresetRank {
        name: "中文说唱榜",
        url: "https://music.163.com/#/discover/toplist?id=991319590",
    },
    PresetRank {
        name: "古典音乐榜",
        url: "https://music.163.com/#/discover/toplist?id=71384707",
    },
    PresetRank {
        name: "电音排行榜",
        url: "https://music.163.com/#/discover/toplist?id=1978921795",
    },
    PresetRank {
        name: "全球说唱榜",
        url: "https://music.163.com/#/discover/toplist?id=14028249541",
    },
    PresetRank {
        name: "潮流风向榜",
        url: "https://music.163.com/#/discover/toplist?id=13372522766",
    },
    PresetRank {
        name: "音乐推荐榜",
        url: "https://music.163.com/#/discover/toplist?id=12911403728",
    },
    PresetRank {
        name: "音乐热歌榜",
        url: "https://music.163.com/#/discover/toplist?id=12911589513",
    },
    PresetRank {
        name: "音乐留名榜",
        url: "https://music.163.com/#/discover/toplist?id=12911619970",
    },
    PresetRank {
        name: "高分新歌榜",
        url: "https://music.163.com/#/discover/toplist?id=12911379734",
    },
    PresetRank {
        name: "音乐高分榜",
        url: "https://music.163.com/#/discover/toplist?id=12768855486",
    },
    PresetRank {
        name: "黑胶爱听榜",
        url: "https://music.163.com/#/discover/toplist?id=5453912201",
    },
    PresetRank {
        name: "ACG排行榜",
        url: "https://music.163.com/#/discover/toplist?id=71385702",
    },
    PresetRank {
        name: "韩语排行榜",
        url: "https://music.163.com/#/discover/toplist?id=745956260",
    },
    PresetRank {
        name: "UK排行榜",
        url: "https://music.163.com/#/discover/toplist?id=180106",
    },
    PresetRank {
        name: "美国Bill榜",
        url: "https://music.163.com/#/discover/toplist?id=60198",
    },
    PresetRank {
        name: "电子舞曲榜",
        url: "https://music.163.com/#/discover/toplist?id=3812895",
    },
    PresetRank {
        name: "KTV唛榜",
        url: "https://music.163.com/#/discover/toplist?id=21845217",
    },
    PresetRank {
        name: "日本Oricon榜",
        url: "https://music.163.com/#/discover/toplist?id=60131",
    },
    PresetRank {
        name: "欧美热歌榜",
        url: "https://music.163.com/#/discover/toplist?id=2809513713",
    },
    PresetRank {
        name: "欧美新歌榜",
        url: "https://music.163.com/#/discover/toplist?id=2809577409",
    },
    PresetRank {
        name: "法国排行榜",
        url: "https://music.163.com/#/discover/toplist?id=27135204",
    },
    PresetRank {
        name: "ACG动画榜",
        url: "https://music.163.com/#/discover/toplist?id=3001835560",
    },
    PresetRank {
        name: "ACG游戏榜",
        url: "https://music.163.com/#/discover/toplist?id=3001795926",
    },
    PresetRank {
        name: "ACG电音榜",
        url: "https://music.163.com/#/discover/toplist?id=3001890046",
    },
    PresetRank {
        name: "日语排行榜",
        url: "https://music.163.com/#/discover/toplist?id=5059644681",
    },
    PresetRank {
        name: "摇滚排行榜",
        url: "https://music.163.com/#/discover/toplist?id=5059633707",
    },
    PresetRank {
        name: "国风排行榜",
        url: "https://music.163.com/#/discover/toplist?id=5059642708",
    },
    PresetRank {
        name: "潜力爆款榜",
        url: "https://music.163.com/#/discover/toplist?id=5338990334",
    },
    PresetRank {
        name: "民谣排行榜",
        url: "https://music.163.com/#/discover/toplist?id=5059661515",
    },
    PresetRank {
        name: "听歌识曲榜",
        url: "https://music.163.com/#/discover/toplist?id=6688069460",
    },
    PresetRank {
        name: "网络热歌榜",
        url: "https://music.163.com/#/discover/toplist?id=6723173524",
    },
    PresetRank {
        name: "俄语排行榜",
        url: "https://music.163.com/#/discover/toplist?id=6732051320",
    },
    PresetRank {
        name: "越南排行榜",
        url: "https://music.163.com/#/discover/toplist?id=6732014811",
    },
    PresetRank {
        name: "慢摇DJ榜",
        url: "https://music.163.com/#/discover/toplist?id=6886768100",
    },
    PresetRank {
        name: "俄语流行榜",
        url: "https://music.163.com/#/discover/toplist?id=6939992364",
    },
    PresetRank {
        name: "泰语排行榜",
        url: "https://music.163.com/#/discover/toplist?id=7095271308",
    },
    PresetRank {
        name: "BEAT排行榜",
        url: "https://music.163.com/#/discover/toplist?id=7356827205",
    },
    PresetRank {
        name: "星云排行榜",
        url: "https://music.163.com/#/discover/toplist?id=7325478166",
    },
    PresetRank {
        name: "直播歌曲榜",
        url: "https://music.163.com/#/discover/toplist?id=7603212484",
    },
    PresetRank {
        name: "赏音排行榜",
        url: "https://music.163.com/#/discover/toplist?id=7775163417",
    },
    PresetRank {
        name: "黑胶新歌榜",
        url: "https://music.163.com/#/discover/toplist?id=7785123708",
    },
    PresetRank {
        name: "黑胶热歌榜",
        url: "https://music.163.com/#/discover/toplist?id=7785066739",
    },
    PresetRank {
        name: "黑胶爱搜榜",
        url: "https://music.163.com/#/discover/toplist?id=7785091694",
    },
    PresetRank {
        name: "实时热度榜",
        url: "https://music.163.com/#/discover/toplist?id=8246775932",
    },
    PresetRank {
        name: "派对潮音榜",
        url: "https://music.163.com/#/discover/toplist?id=8537588450",
    },
    PresetRank {
        name: "乐夏排行榜",
        url: "https://music.163.com/#/discover/toplist?id=8661209031",
    },
    PresetRank {
        name: "特斯拉爱听榜",
        url: "https://music.163.com/#/discover/toplist?id=8703179781",
    },
    PresetRank {
        name: "理想爱听榜",
        url: "https://music.163.com/#/discover/toplist?id=8703052295",
    },
    PresetRank {
        name: "比亚迪爱听榜",
        url: "https://music.163.com/#/discover/toplist?id=8702582160",
    },
    PresetRank {
        name: "蔚来爱听榜",
        url: "https://music.163.com/#/discover/toplist?id=8703220480",
    },
    PresetRank {
        name: "极氪爱听榜",
        url: "https://music.163.com/#/discover/toplist?id=8702982391",
    },
    PresetRank {
        name: "蛋仔派对榜",
        url: "https://music.163.com/#/discover/toplist?id=8532443277",
    },
    PresetRank {
        name: "AI歌曲榜",
        url: "https://music.163.com/#/discover/toplist?id=9651277674",
    },
    PresetRank {
        name: "昊铂爱听榜",
        url: "https://music.163.com/#/discover/toplist?id=10131772880",
    },
    PresetRank {
        name: "埃安爱听榜",
        url: "https://music.163.com/#/discover/toplist?id=10162841534",
    },
    PresetRank {
        name: "欧美R&B榜",
        url: "https://music.163.com/#/discover/toplist?id=12225155968",
    },
    PresetRank {
        name: "黑胶限免榜",
        url: "https://music.163.com/#/discover/toplist?id=12344472377",
    },
    PresetRank {
        name: "吉利爱听榜",
        url: "https://music.163.com/#/discover/toplist?id=12717025277",
    },
];
