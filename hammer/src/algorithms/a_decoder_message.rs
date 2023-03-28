///
/// leetCode 2023/3/8 不能并不能识别方法函数指针，
/// 
/// 
/// 给你字符串 key 和 message ，分别表示一个加密密钥和一段加密消息。解密 message 的步骤如下：
/// 使用 key 中 26 个英文小写字母第一次出现的顺序作为替换表中的字母 顺序 。
/// 将替换表与普通英文字母表对齐，形成对照表。
/// 按照对照表 替换 message 中的每个字母。
/// 空格 ' ' 保持不变。
/// 例如，key = "happy boy"（实际的加密密钥会包含字母表中每个字母 至少一次），据此，可以得到部分对照表（'h' -> 'a'、'a' -> 'b'、'p' -> 'c'、'y' -> 'd'、'b' -> 'e'、'o' -> 'f'）。
/// 返回解密后的消息。
///
///
fn decode_message(key: String, message: String) -> String {

    let compute_char = |c: char, sets: &mut Vec<u8>,  index:&mut usize, keys_bytes: &[u8]| -> char {
        let mut re = c;
        loop {
            if keys_bytes.len() == *index {
                break;
            }
            let src = keys_bytes[*index];
            if src as char == ' ' {
                *index += 1;
            } else if sets.contains(&src) {
                *index += 1;
            } else {
                re = src as char;
                *index += 1;
                sets.push(src);
                break;
            }
        }
        re
    };



    let mut builder = String::from("");
    let mut index = 0;
    let mut sets = Vec::new();
    let keys_bytes = key.as_bytes();

    let a = compute_char('a', &mut sets, &mut index, &keys_bytes);
    let b = compute_char('b', &mut sets, &mut index, &keys_bytes);
    let c = compute_char('c', &mut sets, &mut index, &keys_bytes);
    let d = compute_char('d', &mut sets, &mut index, &keys_bytes);
    let e = compute_char('e', &mut sets, &mut index, &keys_bytes);
    let f = compute_char('f', &mut sets, &mut index, &keys_bytes);
    let g = compute_char('g', &mut sets, &mut index, &keys_bytes);
    let h = compute_char('h', &mut sets, &mut index, &keys_bytes);
    let i = compute_char('i', &mut sets, &mut index, &keys_bytes);
    let j = compute_char('j', &mut sets, &mut index, &keys_bytes);
    let k = compute_char('k', &mut sets, &mut index, &keys_bytes);
    let l = compute_char('l', &mut sets, &mut index, &keys_bytes);
    let m = compute_char('m', &mut sets, &mut index, &keys_bytes);
    let n = compute_char('n', &mut sets, &mut index, &keys_bytes);
    let o = compute_char('o', &mut sets, &mut index, &keys_bytes);
    let p = compute_char('p', &mut sets, &mut index, &keys_bytes);
    let q = compute_char('q', &mut sets, &mut index, &keys_bytes);
    let r = compute_char('r', &mut sets, &mut index, &keys_bytes);
    let s = compute_char('s', &mut sets, &mut index, &keys_bytes);
    let t = compute_char('t', &mut sets, &mut index, &keys_bytes);
    let u = compute_char('u', &mut sets, &mut index, &keys_bytes);
    let v = compute_char('v', &mut sets, &mut index, &keys_bytes);
    let w = compute_char('w', &mut sets, &mut index, &keys_bytes);
    let x = compute_char('x', &mut sets, &mut index, &keys_bytes);
    let y = compute_char('y', &mut sets, &mut index, &keys_bytes);
    let z = compute_char('z', &mut sets, &mut index, &keys_bytes);

    for ele in message.as_bytes() {
        let rc = *ele as char;
        match rc {
            ' ' => builder.push_str(" "),
            ma if ma == a  => builder.push_str("a"),
            mb if mb == b  => builder.push_str("b"),
            mc if mc == c  => builder.push_str("c"),
            md if md == d  => builder.push_str("d"),
            me if me == e  => builder.push_str("e"),
            mf if mf == f  => builder.push_str("f"),
            mg if mg == g  => builder.push_str("g"),
            mh if mh == h  => builder.push_str("h"),
            mi if mi == i  => builder.push_str("i"),
            mj if mj == j  => builder.push_str("j"),
            mk if mk == k  => builder.push_str("k"),
            ml if ml == l  => builder.push_str("l"),
            mm if mm == m  => builder.push_str("m"),
            mn if mn == n  => builder.push_str("n"),
            mo if mo == o  => builder.push_str("o"),
            mp if mp == p  => builder.push_str("p"),
            mq if mq == q  => builder.push_str("q"),
            mr if mr == r  => builder.push_str("r"),
            ms if ms == s  => builder.push_str("s"),
            mt if mt == t  => builder.push_str("t"),
            mu if mu == u  => builder.push_str("u"),
            mv if mv == v  => builder.push_str("v"),
            mw if mw == w  => builder.push_str("w"),
            mx if mx == x  => builder.push_str("x"),
            my if my == y  => builder.push_str("y"),
            mz if mz == z  => builder.push_str("z"),
            _ => print!(""),
        }
    }
    builder
}

#[test]
fn test_decode_message() {
    let decode_msg = decode_message(
        String::from("eljuxhpwnyrdgtqkviszcfmabo"
       ),
        String::from("zwx hnfx lqantp mnoeius ycgk vcnjrdb"),
    );
    assert_eq!("the five boxing wizards jump quickly", decode_msg)
}
