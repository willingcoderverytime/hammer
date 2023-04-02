///
/// 你的赛车可以从位置 0 开始，并且速度为 +1 ，在一条无限长的数轴上行驶。赛车也可以向负方向行驶。赛车可以按照由加速指令 'A' 和倒车指令 'R' 组成的指令序列自动行驶。
/// 当收到指令 'A' 时，赛车这样行驶：
/// position += speed
/// speed *= 2
/// 当收到指令 'R' 时，赛车这样行驶：
/// 如果速度为正数，那么speed = -1
/// 否则 speed = 1
/// 当前所处位置不变。
/// 例如，在执行指令 "AAR" 后，赛车位置变化为 0 --> 1 --> 3 --> 3 ，速度变化为 1 --> 2 --> 4 --> -1 。
///
/// 给你一个目标位置 target ，返回能到达目标位置的最短指令序列的长度。
///
/// 其分析思路为 没一步可达的位置都进行计算，对与规则不合的进行过滤。
/// 即第一步可能到达的点位，  有哪些里面是否包含target，    第二步包含的全部可能。   
/// 为什么不过滤，是为了找到全部的可能。
///
/// 之前的思路完全陷入了定式的数学思维。
///
///
#[derive(PartialEq, Eq, Hash, Clone)]
struct Pos {
    pos: i32,
    speed: i32,
}
#[test]
pub fn test_race_car() {
    let mut testMap = HashMap::new();

    // testMap.insert(1, 1);
    // testMap.insert(2, 4);
    // testMap.insert(3, 2);
    // testMap.insert(4, 5);
    // testMap.insert(5, 7);
    // testMap.insert(6, 5);
    // testMap.insert(7, 3);
    // testMap.insert(8, 6);
    // testMap.insert(9, 8);
    // testMap.insert(10, 7);
    // testMap.insert(11, 10);
    // testMap.insert(12, 7);
    // testMap.insert(13, 9);
    // testMap.insert(14, 6);
    // testMap.insert(15, 4);
    testMap.insert(20, 12);

    for (key, value) in testMap.into_iter() {
        println!("target {} cast step {}", key, value);
        assert_eq!(value, racecar(key));
    }
}
use std::collections::{HashMap, HashSet};
pub fn racecar(target: i32) -> i32 {
    let mut visited: HashSet<Pos> = HashSet::new();

    let mut cur_layer: Vec<Pos> = Vec::new();

    let start_pos = Pos { pos: 0, speed: 1 };
    cur_layer.push(start_pos);
    let mut ret = 0;
    while !cur_layer.is_empty() {
        let mut next_layer: Vec<Pos> = Vec::new();
        for pos in cur_layer.iter() {
            if visited.contains(pos) {
                continue;
            }
            visited.insert(pos.clone());
            if pos.pos == target {
                return ret;
            }
            // receive A
            let mut new_pos = Pos {
                pos: pos.pos + pos.speed,
                speed: pos.speed * 2,
            };
            next_layer.push(new_pos);
            //receive R
            new_pos = Pos {
                pos: pos.pos,
                speed: if pos.speed > 0 { -1 } else { 1 },
            };
            next_layer.push(new_pos);
        }
        cur_layer = next_layer;
        ret += 1;
    }

    return ret;
}
