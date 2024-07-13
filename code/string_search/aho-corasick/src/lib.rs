use std::collections::{BTreeMap, VecDeque};

type StateId = usize;
type PatternId = usize;

const ROOT_STATE: StateId = 0;

#[derive(Debug)]
pub struct AcAutomaton {
    patterns: Vec<String>,
    states: Vec<State>,
}

#[derive(Default, Debug)]
struct State {
    next: BTreeMap<char, StateId>,
    fail: StateId,
    // The patterns that are matched at this state
    // Including the patterns that will be matched at the fail state
    matched: Vec<PatternId>,
}

impl AcAutomaton {
    pub fn new<S: Into<String>>(patterns: Vec<S>) -> Self {
        AcAutomaton {
            patterns: Vec::new(),
            states: vec![State::default()],
        }
        .build(patterns.into_iter().map(Into::into).collect())
    }

    fn build(mut self, patterns: Vec<String>) -> Self {
        for (idx, pattern) in patterns.iter().enumerate() {
            let mut curr_state = ROOT_STATE;
            for char in pattern.chars() {
                curr_state = match self.next_of_state(curr_state, char) {
                    Some(next_state) => next_state,
                    None => {
                        let next_state = self.add_state(State::default());
                        self.add_next_of_state(curr_state, char, next_state);
                        next_state
                    }
                };
            }

            self.states[curr_state].matched.push(idx as PatternId);
        }

        self.patterns = patterns;
        self.fill()
    }

    fn add_state(&mut self, state: State) -> StateId {
        let id = self.states.len();
        self.states.push(state);
        id
    }

    fn next_of_state(&self, state_id: StateId, char: char) -> Option<StateId> {
        self.states[state_id].next.get(&char).copied()
    }

    fn add_next_of_state(&mut self, state_id: StateId, char: char, next_state_id: StateId) {
        self.states[state_id].next.insert(char, next_state_id);
    }

    fn fail_state(&self, state_id: StateId) -> StateId {
        self.states[state_id].fail
    }

    fn fill(mut self) -> Self {
        let mut queue: VecDeque<StateId> = VecDeque::new();
        for (_, &next_state) in self.states[ROOT_STATE].next.iter() {
            queue.push_back(next_state);
        }

        // Let the first level nodes' fail point to the root node
        for &next_state in queue.iter() {
            self.states[next_state].fail = ROOT_STATE;
        }

        while let Some(curr_state) = queue.pop_front() {
            let mut fill_state = Vec::new();
            for (&char, &next_state) in self.states[curr_state].next.iter() {
                queue.push_back(next_state);
                let mut fail_state = self.states[curr_state].fail;

                while self.next_of_state(fail_state, char).is_none() {
                    // If root state is reached, and no match is found
                    // then the fail state of the current state is the root state
                    if fail_state == ROOT_STATE {
                        break;
                    }
                    fail_state = self.fail_state(fail_state);
                }

                let fail_state = self.next_of_state(fail_state, char).unwrap_or(ROOT_STATE);
                fill_state.push((next_state, fail_state));
            }

            while let Some((state, fail_state)) = fill_state.pop() {
                self.states[state].fail = fail_state;
                let matched_fail = self.states[fail_state].matched.clone();
                self.states[state].matched.extend(matched_fail); // substring is also matched in the fail state
            }
        }

        self
    }

    fn find<'s>(&'s self, text: &'s str) -> Matcher<'_, 's> {
        Matcher {
            automaton: self,
            text,
            idx: 0,
            state: ROOT_STATE,
            matched_idx: 0,
        }
    }
}

struct Matcher<'a, 's> {
    automaton: &'a AcAutomaton,
    text: &'s str,
    idx: usize,             // Current index of the text
    state: StateId,         // Current state of the automaton
    matched_idx: PatternId, // Usefull when different patterns are matched at the same index
}

#[derive(Debug, PartialEq)]
struct Match {
    start: usize,
    end: usize,
    pati: PatternId,
}

impl<'a, 's> Iterator for Matcher<'a, 's> {
    type Item = Match;
    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.text.len() {
            if self.has_match() {
                let match_ = self.get_match();
                self.matched_idx += 1;
                return Some(match_);
            }

            self.matched_idx = 0; // No match found, reset the matched index
            self.idx += 1;
            if self.idx == self.text.len() {
                break;
            }
            self.state = self.next_state();
        }

        None
    }
}

impl<'a, 's> Matcher<'a, 's> {
    fn next_state(&self) -> StateId {
        let char = self.text.chars().nth(self.idx).unwrap();
        // If reach leaf node, find in the fail state
        let mut state = self.state;
        loop {
            match self.automaton.next_of_state(state, char) {
                Some(next_state) => return next_state,
                None => {
                    if state == ROOT_STATE {
                        return ROOT_STATE;
                    }
                    state = self.automaton.fail_state(state);
                }
            }
        }
    }

    fn has_match(&self) -> bool {
        let matched = &self.automaton.states[self.state].matched;
        self.matched_idx < matched.len()
    }

    fn get_match(&self) -> Match {
        let matched = &self.automaton.states[self.state].matched;
        let pati = matched[self.matched_idx];
        let pat = &self.automaton.patterns[pati];
        Match {
            start: self.idx + 1 - pat.len(),
            end: self.idx + 1,
            pati,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::{AcAutomaton, Match};

    fn aut_find<S: Into<String>>(xs: Vec<S>, haystack: S) -> Vec<Match> {
        AcAutomaton::new(xs).find(&haystack.into()).collect()
    }

    #[test]
    fn one_pattern_one_match() {
        let ns = vec!["a"];
        let hay = "za";
        assert_eq!(
            aut_find(ns, hay),
            vec![Match {
                pati: 0,
                start: 1,
                end: 2
            },]
        );
    }

    #[test]
    fn one_pattern_many_match() {
        let ns = vec!["a"];
        let hay = "zazazzzza";
        assert_eq!(
            aut_find(ns, hay),
            vec![
                Match {
                    pati: 0,
                    start: 1,
                    end: 2
                },
                Match {
                    pati: 0,
                    start: 3,
                    end: 4
                },
                Match {
                    pati: 0,
                    start: 8,
                    end: 9
                },
            ]
        );
    }

    #[test]
    fn one_longer_pattern_one_match() {
        let ns = vec!["abc"];
        let hay = "zazabcz";
        assert_eq!(
            aut_find(ns, hay),
            vec![Match {
                pati: 0,
                start: 3,
                end: 6
            },]
        );
    }

    #[test]
    fn one_longer_pattern_many_match() {
        let ns = vec!["abc"];
        let hay = "zazabczzzzazzzabc";
        assert_eq!(
            aut_find(ns, hay),
            vec![
                Match {
                    pati: 0,
                    start: 3,
                    end: 6
                },
                Match {
                    pati: 0,
                    start: 14,
                    end: 17
                },
            ]
        );
    }

    #[test]
    fn many_pattern_one_match() {
        let ns = vec!["a", "b"];
        let hay = "zb";
        assert_eq!(
            aut_find(ns, hay),
            vec![Match {
                pati: 1,
                start: 1,
                end: 2
            },]
        );
    }

    #[test]
    fn many_pattern_many_match() {
        let ns = vec!["a", "b"];
        let hay = "zbzazzzzb";
        assert_eq!(
            aut_find(ns, hay),
            vec![
                Match {
                    pati: 1,
                    start: 1,
                    end: 2
                },
                Match {
                    pati: 0,
                    start: 3,
                    end: 4
                },
                Match {
                    pati: 1,
                    start: 8,
                    end: 9
                },
            ]
        );
    }

    #[test]
    fn many_longer_pattern_one_match() {
        let ns = vec!["abc", "xyz"];
        let hay = "zazxyzz";
        assert_eq!(
            aut_find(ns, hay),
            vec![Match {
                pati: 1,
                start: 3,
                end: 6
            },]
        );
    }

    #[test]
    fn many_longer_pattern_many_match() {
        let ns = vec!["abc", "xyz"];
        let hay = "zazxyzzzzzazzzabcxyz";
        assert_eq!(
            aut_find(ns, hay),
            vec![
                Match {
                    pati: 1,
                    start: 3,
                    end: 6
                },
                Match {
                    pati: 0,
                    start: 14,
                    end: 17
                },
                Match {
                    pati: 1,
                    start: 17,
                    end: 20
                },
            ]
        );
    }

    #[test]
    fn many_longer_pattern_overlap_one_match() {
        let ns = vec!["abc", "bc"];
        let hay = "zazabcz";
        assert_eq!(
            aut_find(ns, hay),
            vec![
                Match {
                    pati: 0,
                    start: 3,
                    end: 6
                },
                Match {
                    pati: 1,
                    start: 4,
                    end: 6
                },
            ]
        );
    }

    #[test]
    fn many_longer_pattern_overlap_one_match_reverse() {
        let ns = vec!["abc", "bc"];
        println!("{:?}", AcAutomaton::new(ns.clone()));
        let hay = "xbc";
        assert_eq!(
            aut_find(ns, hay),
            vec![Match {
                pati: 1,
                start: 1,
                end: 3
            },]
        );
    }

    #[test]
    fn many_longer_pattern_overlap_many_match() {
        let ns = vec!["abc", "bc", "c"];
        let hay = "zzzabczzzbczzzc";
        assert_eq!(
            aut_find(ns, hay),
            vec![
                Match {
                    pati: 0,
                    start: 3,
                    end: 6
                },
                Match {
                    pati: 1,
                    start: 4,
                    end: 6
                },
                Match {
                    pati: 2,
                    start: 5,
                    end: 6
                },
                Match {
                    pati: 1,
                    start: 9,
                    end: 11
                },
                Match {
                    pati: 2,
                    start: 10,
                    end: 11
                },
                Match {
                    pati: 2,
                    start: 14,
                    end: 15
                },
            ]
        );
    }

    #[test]
    fn many_longer_pattern_overlap_many_match_reverse() {
        let ns = vec!["abc", "bc", "c"];
        let hay = "zzzczzzbczzzabc";
        assert_eq!(
            aut_find(ns, hay),
            vec![
                Match {
                    pati: 2,
                    start: 3,
                    end: 4
                },
                Match {
                    pati: 1,
                    start: 7,
                    end: 9
                },
                Match {
                    pati: 2,
                    start: 8,
                    end: 9
                },
                Match {
                    pati: 0,
                    start: 12,
                    end: 15
                },
                Match {
                    pati: 1,
                    start: 13,
                    end: 15
                },
                Match {
                    pati: 2,
                    start: 14,
                    end: 15
                },
            ]
        );
    }

    #[test]
    fn weird_dna() {
        let ns = vec!["tttacccc"];
        println!("{:?}", AcAutomaton::new(ns.clone()));
        let hay = "ttttacccc";
        assert_eq!(
            aut_find(ns, hay),
            vec![Match {
                pati: 0,
                start: 1,
                end: 9
            },]
        );
    }
}
