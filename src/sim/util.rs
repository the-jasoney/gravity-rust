pub fn reduce2d<T>(v: Vec<Vec<T>>) -> Vec<T>
where
    T: PartialEq,
{
    let mut reduced: Vec<T> = vec![];

    for i in v {
        for j in i {
            if !reduced.contains(&j) {
                reduced.push(j);
            }
        }
    }

    reduced
}
