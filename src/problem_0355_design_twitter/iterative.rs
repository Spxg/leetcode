use std::collections::{HashMap, HashSet};

struct Twitter {
    priority: i32,
    users: HashMap<i32, HashSet<i32>>,
    tweets: HashMap<i32, Vec<i32>>,
    time: HashMap<i32, i32>,
}

impl Twitter {
    fn new() -> Self {
        Self {
            priority: 0,
            users: HashMap::new(),
            tweets: HashMap::new(),
            time: HashMap::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.users.entry(user_id).or_default();
        self.tweets.entry(user_id).or_default().push(tweet_id);
        self.time.insert(tweet_id, self.priority);
        self.priority += 1;
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        if !self.users.contains_key(&user_id) {
            return vec![];
        }

        let followee = &self.users[&user_id];
        let mut tweets = Vec::with_capacity(followee.len() + 1);
        for uid in followee.iter().chain(&[user_id]) {
            if let Some(x) = self.tweets.get(uid).map(std::vec::Vec::as_slice) {
                tweets.push(x);
            }
        }
        let mut result = Vec::new();

        loop {
            let mut new = (-1, 0);
            (0..tweets.len()).for_each(|idx| {
                if let Some(tweet) = tweets[idx].last().copied() {
                    let priority = self.time[&tweet];
                    if priority > new.0 {
                        new = (priority, idx);
                    }
                }
            });
            if result.len() == 10 || new.0 == -1 {
                break;
            }
            let (&last, rest) = tweets[new.1].split_last().unwrap();
            result.push(last);
            tweets[new.1] = rest;
        }
        result
    }

    #[allow(clippy::similar_names)]
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.users
            .entry(follower_id)
            .or_default()
            .insert(followee_id);
    }

    #[allow(clippy::similar_names)]
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.users
            .get_mut(&follower_id)
            .unwrap()
            .remove(&followee_id);
    }
}

impl super::Twitter for Twitter {
    fn new() -> Self {
        Self::new()
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.post_tweet(user_id, tweet_id);
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        self.get_news_feed(user_id)
    }

    #[allow(clippy::similar_names)] // Expected.
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follow(follower_id, followee_id);
    }

    #[allow(clippy::similar_names)] // Expected.
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.unfollow(follower_id, followee_id);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Twitter>();
    }
}
