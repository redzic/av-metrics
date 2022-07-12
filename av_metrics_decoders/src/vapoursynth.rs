use vapoursynth::node::{GetFrameError, Node};

use vapoursynth::prelude::*;

/// Vapoursynth decoder interface
pub struct VapoursynthDecoder<'a> {
    frame_idx: usize,
    node: Node<'a>,
    bit_depth: usize,
}

/// Vapoursynth error
pub enum VapoursynthError {
    /// VsScript error
    VsScript(vsscript::Error),
}

impl<'a> VapoursynthDecoder<'a> {
    // TODO return error instead
    fn new(env: &'a Environment) -> Result<Self, vsscript::Error> {
        const OUTPUT_INDEX: i32 = 0;

        #[cfg(feature = "vapoursynth_new_api")]
        let (node, _) = env.get_output(OUTPUT_INDEX)?;
        #[cfg(not(feature = "vapoursynth_new_api"))]
        let node = env.get_output(OUTPUT_INDEX)?;

        let bit_depth = match node.info().format {
            Property::Variable => {
                panic!("Cannot output clips with variable format");
            }
            Property::Constant(x) => x.bits_per_sample(),
        };

        Ok(Self {
            frame_idx: 0,
            node,
            bit_depth: bit_depth as usize,
        })
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
