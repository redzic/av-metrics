use av_metrics::video::decode::VideoDetails;
use vapoursynth::core::CoreRef;
use vapoursynth::node::Node;

use vapoursynth::prelude::*;

/// Vapoursynth decoder interface
pub struct VapoursynthDecoder<'a> {
    frame_idx: usize,
    node: Node<'a>,
    video_details: VideoDetails,
    pub core: CoreRef<'a>,
}

/// Vapoursynth error
#[derive(Debug)]
pub enum VapoursynthError {
    /// VsScript error
    VsScript(vsscript::Error),
    /// Script has variable format in output
    VariableFormat,
    /// Script has variable resolution in output
    VariableResolution,
}

impl From<vsscript::Error> for VapoursynthError {
    fn from(e: vsscript::Error) -> Self {
        Self::VsScript(e)
    }
}

impl<'a> VapoursynthDecoder<'a> {
    pub fn new(env: &'a Environment) -> Result<Self, VapoursynthError> {
        const OUTPUT_INDEX: i32 = 0;

        #[cfg(feature = "vapoursynth_new_api")]
        let (node, _) = env.get_output(OUTPUT_INDEX)?;
        #[cfg(not(feature = "vapoursynth_new_api"))]
        let node = env.get_output(OUTPUT_INDEX)?;

        let bit_depth = match node.info().format {
            Property::Variable => {
                return Err(VapoursynthError::VariableFormat);
            }
            Property::Constant(x) => x.bits_per_sample(),
        };

        let resolution = match node.info().resolution {
            Property::Variable => return Err(VapoursynthError::VariableResolution),
            Property::Constant(x) => x,
        };

        let video_details = VideoDetails {
            bit_depth: bit_depth as usize,
            width: resolution.width,
            height: resolution.height,
            // TODO actually report this properly
            ..Default::default()
        };

        // TODO error handling
        let core = env.get_core().unwrap();

        Ok(Self {
            frame_idx: 0,
            node,
            video_details,
            core,
        })
    }

    pub fn get_bit_depth(&self) -> usize {
        self.video_details.bit_depth
    }

    // TODO write the safety contracts
    pub fn receive_frame_init<'b>(&'b mut self) -> Option<FrameRef<'a>> {
        let frame = self.node.get_frame(self.frame_idx);

        self.frame_idx += 1;

        frame.ok()
    }

    pub fn receive_frame<'b>(&'b mut self, x: &'b mut FrameRef<'a>) -> bool {
        let frame = self.node.get_frame(self.frame_idx);

        self.frame_idx += 1;

        if let Ok(frame) = frame {
            *x = frame;
            true
        } else {
            false
        }
    }

    pub fn get_video_details(&self) -> VideoDetails {
        self.video_details
    }
}
