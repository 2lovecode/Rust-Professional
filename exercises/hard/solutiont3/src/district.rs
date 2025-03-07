use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
pub fn count_provinces() -> String {

    // 从文件中读取数据
    let district_path = Path::new("district.json");
    let content = fs::read_to_string(district_path).unwrap();

    // 解析数据
    let mut list: Vec<HashMap<String, Vec<String>>> = Vec::new();
    let kv = parse(content.clone());
    let mut kvec: Vec<(String, String)> = kv.into_iter().collect();
    kvec.sort_by_key(|(k, _)| k.to_string());
    kvec.iter().for_each(|(k, v)| {
        let mut hm = HashMap::new();

        let kv2 = parse(v.clone());
        kv2.iter().for_each(|(k2, v2)| {
            let v2b = v2.trim().trim_end_matches(",").trim_start_matches("[").trim_end_matches("]").to_string();
            let v2l: Vec<String> = v2b.split(",").map(|s| s.trim().trim_start_matches('"').trim_end_matches('"').to_string()).collect();

            let mut uniq:HashSet<String> = HashSet::new();

            let mut v2ll: Vec<String> = Vec::new();

            v2l.into_iter().for_each(|s| {
                if !uniq.contains(&s) {
                    v2ll.push(s.clone());
                    uniq.insert(s.clone());
                }
            });
            // println!("{}", v2ll.join(","));

            hm.insert(k2.trim().trim_start_matches('"').trim_end_matches('"').to_string(), v2ll);
        });
        list.push(hm);
    });

    // 构建图结构
    let mut graphs: Vec<HashMap<String, Vec<String>>> = Vec::new();

    list.iter().for_each(|hm| {
        let mut n:HashMap<String, Vec<String>> = HashMap::new();

        hm.iter().for_each(|(k, v)| {

            if let Some(vec) = n.get_mut(k) {
                for v2  in v {
                    vec.push(v2.to_string());
                }
            } else {
                n.insert(k.to_string(), v.clone());
            }
            for v2 in v {
                if let Some(vec) = n.get_mut(v2) {
                    vec.push(k.to_string());
                } else {
                    n.insert(v2.to_string(), vec![k.to_string()]);
                }
            }
        });
        graphs.push(n);
    });

    // 遍历图
    let mut counts: Vec<i32> = Vec::new();

    graphs.iter().for_each(|hm| {
        let mut visited: HashMap<String, bool> = HashMap::new();
        let mut queue: Vec<String> = Vec::new();
        let mut cnt = 0;
        for (k, v) in hm {
            if visited.contains_key(k) {
                continue;
            }
            queue.push(k.to_string());
            cnt += 1;
            while !queue.is_empty() {
                let vv = queue.remove(0);
                if !visited.contains_key(&vv) {
                    visited.insert(vv.to_string(), true);
                    hm.get(&vv).unwrap().iter().for_each(|v2| {
                        if !visited.contains_key(v2) {
                            queue.push(v2.to_string());
                        }
                    });
                }
            }

        }
        
        counts.push(cnt);
    });

    counts.iter()
        .fold(String::new(), |mut acc, &x| {
            if !acc.is_empty() {
                acc.push(',');
            }
            acc.push_str(&x.to_string());
            acc
        })
}

fn parse(value: String) -> HashMap<String, String> {
    let mut v_t = value.trim();
    if v_t.starts_with("{") {
        v_t = v_t.trim_start_matches("{").trim_end_matches("}");
    } else if v_t.starts_with("[") {
        v_t = v_t.trim_start_matches("[").trim_end_matches("]");
    }
    
    let mut v_chars: Vec<char> = v_t.chars().collect();

    let mut kl: Vec<char> = Vec::new();
    let mut vl: Vec<char> = Vec::new();
    let mut queue: Vec<char> = Vec::new();

    let mut hm: HashMap<String, String> = HashMap::new();

    let mut is_key = true;
    let mut valid = false;
    for i in 0..v_chars.len() {
        let c = v_chars[i];
        if is_key {
            if c == ':' {
                is_key = false;
                continue;
            }
            kl.push(c);
        } else {
            vl.push(c);
            if c == '{' || c == '[' {
                valid = true;
                queue.push(c);
            }
            if valid {
                if c == '}' {
                    let p = queue.pop().unwrap();
                    if p == '{' {
                        continue;
                    } else {
                        break;
                    }
                } else if c == ']' {
                    let p = queue.pop().unwrap();
                    if p == '[' {
                        continue;
                    } else {
                        break;
                    }
                }
                if queue.is_empty() {
                    let kk = kl.into_iter().collect::<String>().trim().trim_matches('"').trim_end_matches(',').trim_end_matches('"').to_string();
                    let vv = vl.into_iter().collect::<String>().trim().trim_end_matches(",").to_string();
                    

                    if let Some(vvv) = hm.get_mut(&kk) {
                        

                        if vvv.starts_with("[") && vv.starts_with("[") {
                            let mut new_value = vvv.trim_end_matches(']').to_string();
                            new_value.push(',');
                            new_value.push_str(vv.trim_start_matches('['));
                    
                            *vvv = new_value;
                        }
                        // println!("k: {}, v:{}", kk, vvv);
                    } else {
                        hm.insert(kk, vv);
                    }
                    kl = Vec::new();
                    vl = Vec::new();
                    is_key = true;
                    valid = false;
                    continue;
                }
            }
        }   
    }
    hm
}
