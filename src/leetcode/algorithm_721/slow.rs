use std::collections::{HashMap, HashSet};
pub struct Solution;

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut names = vec![];
        let mut emails: HashMap<String, usize> = HashMap::new();
        for (i, name_emails) in accounts.into_iter().enumerate() {
            let mut id = i;
            let (name, account_emails) = name_emails.split_first().unwrap();
            names.push(vec![name.clone()]);
            for email in name_emails.iter() {
                if let Some(&component_id) = emails.get(email) {
                    id = component_id;
                    break;
                }
            }
            let mut to_modify = HashSet::new();
            for email in account_emails.iter() {
                if let Some(existing) = emails.get_mut(email) {
                    to_modify.insert(*existing);
                    *existing = id;
                    continue;
                }
                emails.insert(email.clone(), id);
            }
            for (_, v) in emails.iter_mut() {
                if to_modify.contains(v) {
                    *v = id;
                }
            }
        }
        for (email, i) in emails.into_iter() {
            names[i].push(email);
        }
        names
            .into_iter()
            .filter(|accounts| accounts.len() > 1)
            .map(|mut accounts| {
                let (_, emails) = accounts.split_first_mut().unwrap();
                emails.sort_unstable();
                accounts
            })
            .collect()
    }
}
