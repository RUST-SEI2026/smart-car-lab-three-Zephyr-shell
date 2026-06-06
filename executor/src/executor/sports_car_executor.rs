pub(crate) use crate::assembler::SportsCarState;
use crate::{Executor, Pose};
pub struct SportsCarExecutor;
impl SportsCarExecutor{
    pub fn with_pose(pose: Pose) -> Executor{
        Executor{
            pose,
            state:Box::new(SportsCarState::default()),
        }
    }

    // pub fn executor(&mut self, cmds:&str){
    //     todo!();
    // }

    // pub fn query(&self) -> Pose{
    //     self.pose
    // }
}
