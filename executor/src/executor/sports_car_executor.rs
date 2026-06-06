
use crate::{Executor, Pose};
use super::super::sports_car_state::SportsCarState;
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
