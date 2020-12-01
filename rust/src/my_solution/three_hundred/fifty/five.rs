use std::collections::{HashMap, HashSet};

///设计一个简化版的推特(Twitter)，可以让用户实现发送推文，关注/取消关注其他用户，能够看见关注人（包括自己）的最近十条推文。你的设计需要支持以下的几个功能：
//
// postTweet(userId, tweetId): 创建一条新的推文
// getNewsFeed(userId): 检索最近的十条推文。每个推文都必须是由此用户关注的人或者是用户自己发出的。推文必须按照时间顺序由最近的开始排序。
// follow(followerId, followeeId): 关注一个用户
// unfollow(followerId, followeeId): 取消关注一个用户
//
// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/design-twitter
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
#[derive(Debug)]
pub struct Twitter {
    id: i32,
    follow_list: HashMap<i32, HashSet<i32>>,
    tweet_list: HashMap<i32, HashSet<(i32, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            id: 0,
            follow_list: Default::default(),
            tweet_list: Default::default(),
        }
    }

    /** Compose a new tweet. */
    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.id += 1;
        self.tweet_list
            .entry(user_id)
            .or_default()
            .insert((self.id, tweet_id));
    }

    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    pub fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut res: HashSet<(i32, i32)> = HashSet::new();
        if let Some(t) = self.follow_list.get(&user_id) {
            for i in t {
                if let Some(tt) = self.tweet_list.get(i) {
                    res.extend(tt.iter());
                }
            }
        }
        if let Some(t) = self.tweet_list.get(&user_id) {
            res.extend(t.iter());
        }
        let mut res = res.iter().collect::<Vec<_>>();
        res.sort_by(|a, b| b.0.cmp(&a.0));
        if res.len() > 10 {
            res.drain(10..);
        }

        res.iter().map(|r| r.1).collect()
    }

    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follow_list
            .entry(follower_id)
            .or_default()
            .insert(followee_id);
    }

    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.follow_list
            .entry(follower_id)
            .or_default()
            .remove(&followee_id);
    }
}
