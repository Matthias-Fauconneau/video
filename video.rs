fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let path = &std::env::args().skip(1).next().unwrap();
    let map = unsafe{memmap::Mmap::map(&std::fs::File::open(path)?)}?;
    let input = &*map;
    let mut demuxer = matroska::demuxer::MkvDemuxer::new();
    let (input, ()) = demuxer.parse_until_tracks(input).unwrap();
    let elements = {let mut input=input; std::iter::from_fn(move ||{let (rest, element) = matroska::elements::segment_element(input).unwrap(); input = rest; Some(element)})};
    let tracks = demuxer.tracks.unwrap();
    let video = &tracks.tracks[0];
    assert!(video.codec_id == "V_MPEGH/ISO/HEVC");
    //video.codec_private : ISO.14496-15::HEVCDecoderConfigurationRecord
    for element in elements { use  matroska::elements::SegmentElement::*; match element {
        Void(_) => {},
        Cluster(cluster) => for data_block in cluster.simple_block {
            let (data, block) = matroska::elements::simple_block(data_block).unwrap();
            if block.track_number == video.track_number {
                panic!("{}", data.len());
            }
        },
        Cues(_) => {},
        _ => panic!("{element:?}")
    }}
    Ok(())
}
