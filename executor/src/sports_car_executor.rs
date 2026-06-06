
use crate::Pose;

pub struct SportsCarExecutor{
    pose: Pose,
}

impl SportsCarExecutor{
    pub fn with_pose(pose: Pose) -> Self{
        todo!();
    }

    pub fn executor(&mut self, cmds:&str){
        todo!();
    }

    pub fn query(&self) -> Pose{
        self.pose
    }
}
