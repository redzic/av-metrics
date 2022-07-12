use std::path::Path;

use vapoursynth::node::{GetFrameError, Node};

use vapoursynth::prelude::*;

/// Vapoursynth decoder interface
pub struct VapoursynthDecoder<'a> {
    frame_idx: usize,
    node: Node<'a>,
    bit_depth: usize,
}

impl<'a> VapoursynthDecoder<'a> {
    // TODO return error instead
    fn new(env: &'a Environment) -> Self {
        let node = env.get_output(0).unwrap();

        // let frame0 = node.get_frame(0).unwrap();

        // env.get_output(0).unwrap().0.info().format.bits_per_sample();
        let bit_depth = match env.get_output(0).unwrap().0.info().format {
            Property::Variable => {
                panic!("Cannot output clips with variable format");
            }
            Property::Constant(x) => x.bits_per_sample(),
        };

        Self {
            frame_idx: 0,
            node: node.0,
            bit_depth: bit_depth as usize,
        }
    }

    fn get_bit_depth(&self) -> usize {
        self.bit_depth
    }

    fn receive_frame(&mut self) -> Result<FrameRef, GetFrameError> {
        let frame = self.node.get_frame(self.frame_idx);

        self.frame_idx += 1;

        frame
    }
}
