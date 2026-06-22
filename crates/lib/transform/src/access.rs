use crate::access_pair::AccessPair;

#[derive(Debug, Clone)]
pub struct TransformAccess {
    pub mount: String,
    pub input: Vec<AccessPair>,
    pub output: Vec<AccessPair>,
}

impl TransformAccess {
    pub fn new(mount: &String, input: &Vec<AccessPair>, output: &Vec<AccessPair>) -> Self {
        Self {
            mount: mount.clone(),
            input: input.clone(),
            output: output.clone(),
        }
    }

    pub fn mount(
        desired: &String,
        mount: &String,
        input: Vec<AccessPair>,
        output: Vec<AccessPair>,
    ) -> TransformAccess {
        let mount_point: Vec<&str> = mount.split("<mount>/").collect();
        let mount_point = mount_point.join(desired);
        let mut input = input.clone();
        let mut output = output.clone();
        for i in 0..input.len() {
            input[i].0 = input[i].0.replace("<mount>/", &mount_point);
        }
        for o in 0..output.len() {
            output[o].0 = output[o].0.replace("<mount>/", &mount_point);
        }
        TransformAccess::new(&mount_point.to_string(), &input, &output)
    }
}
