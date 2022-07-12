use av_metrics::video::decode::VideoDetails;
use vapoursynth::node::{GetFrameError, Node};

use vapoursynth::prelude::*;

/// Vapoursynth decoder interface
pub struct VapoursynthDecoder<'a> {
    frame_idx: usize,
    node: Node<'a>,
    video_details: VideoDetails,
}

impl<'a> VapoursynthDecoder<'a> {
    // TODO return error instead
    pub fn new(env: &'a Environment) -> Self {
        let (node, _) = env.get_output(0).unwrap();

        let bit_depth = match node.info().format {
            Property::Variable => {
                panic!("Cannot output clips with variable format");
            }
            Property::Constant(x) => x.bits_per_sample(),
        };

        let resolution = match node.info().resolution {
            Property::Variable => panic!(),
            Property::Constant(x) => x,
        };

        let video_details = VideoDetails {
            bit_depth: bit_depth as usize,
            width: resolution.width,
            height: resolution.height,
            // TODO actually report this properly
            ..Default::default()
        };

        Self {
            frame_idx: 0,
            node,
            video_details,
        }
    }

    pub fn get_bit_depth(&self) -> usize {
        // self.bit_depth
        self.video_details.bit_depth
    }

    pub fn receive_frame<'b>(&'b mut self) -> Result<FrameRef<'a>, GetFrameError> {
        // pub fn receive_frame(&mut self, x: &mut FrameRef) -> bool {
        let frame = self.node.get_frame(self.frame_idx);

        self.frame_idx += 1;

        frame
    }

    pub fn get_video_details(&self) -> VideoDetails {
        self.video_details
    }
}
