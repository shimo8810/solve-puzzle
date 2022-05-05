use std::collections::{HashMap, HashSet, VecDeque};

const EMPTY: u8 = 1;
const MAN: u8 = 2;
const WOLF: u8 = 3;
const GOAT: u8 = 5;
const CABBAGE: u8 = 7;
const ALL: u8 = MAN * WOLF * GOAT * CABBAGE;

fn get_right(left: u8) -> u8 {
    ALL / left
}

fn move_passenger(state: [u8; 2], passenger: u8) -> [u8; 2] {
    if state[0] % passenger == 0 {
        [state[0] / passenger, state[1] * passenger]
    } else {
        [state[0] * passenger, state[1] / passenger]
    }
}

fn exist(side: u8, passenger: u8) -> bool {
    side % passenger == 0
}

fn decode(left: u8) -> String {
    let right = get_right(left);
    let ps = [MAN, WOLF, GOAT, CABBAGE];
    let num2name = HashMap::from([(2, "Man"), (3, "Wolf"), (5, "Goat"), (7, "Cabbage")]);

    let left = ps
        .iter()
        .filter(|&p| left % p == 0)
        .map(|p| num2name.get(p).unwrap().to_string())
        .collect::<Vec<String>>()
        .join(",");

    let right = ps
        .iter()
        .filter(|&p| right % p == 0)
        .map(|p| num2name.get(p).unwrap().to_string())
        .collect::<Vec<String>>()
        .join(",");
    format!("({}) -- ({})", left, right)
}

fn stay_together(side: u8, p1: u8, p2: u8) -> bool {
    side % (p1 * p2) == 0
}

pub fn solve() -> Vec<String> {
    let mut que = VecDeque::from([ALL]);
    let mut visited = HashSet::new();
    let mut prev = HashMap::new();
    let mut ans = vec![];

    while let Some(left) = que.pop_front() {
        // 終了条件
        if left == EMPTY {
            ans.push(decode(left));
            let mut node = left;
            while let Some(&state) = prev.get(&node) {
                ans.push(decode(state));
                node = state;
            }
            return ans.into_iter().rev().collect();
        }

        // 到達状況の確認
        visited.insert(left);

        // 対岸の状況を取得
        let mut state = [left, get_right(left)];
        // 男のいる場所の取得
        let idx = if exist(left, MAN) { 0 } else { 1 };

        state = move_passenger(state, MAN);

        // 男だけが移動するパターンを記録
        if !stay_together(state[idx], WOLF, GOAT)
            && !stay_together(state[idx], GOAT, CABBAGE)
            && !visited.contains(&state[0])
        {
            que.push_back(state[0]);
            prev.insert(state[0], left);
        }

        for p in [WOLF, GOAT, CABBAGE] {
            if exist(state[idx], p) {
                let next_state = move_passenger(state, p);

                if !stay_together(next_state[idx], WOLF, GOAT)
                    && !stay_together(next_state[idx], GOAT, CABBAGE)
                    && !visited.contains(&next_state[0])
                {
                    que.push_back(next_state[0]);
                    prev.insert(next_state[0], left);
                }
            }
        }
    }

    ans
}
