use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct StoryOption {
    pub next_node_id: u32,
    pub text: String,
    pub is_available: bool,
}

#[derive(Debug, Clone)]
pub struct StoryNode {
    pub id: u32,
    pub title: String,
    pub text: String,
    pub options: Vec<StoryOption>,
    pub is_end_node: bool,
    pub is_start_node: bool,
    pub previous_node_id: Option<u32>,
    pub next_node_id: Option<u32>,
}

impl StoryNode {
    pub fn new(id: u32, title: String, text: String, options: Vec<StoryOption>, is_end_node: bool, is_start_node: bool) -> Self {

        let previous_node_id = if is_start_node {
            None
        } else {
            Some(id - 1)
        };

        let next_node_id = if is_end_node {
            None
        } else {
            Some(id + 1)
        };

        StoryNode {
            id,
            title,
            text,
            options,
            is_end_node,
            is_start_node,
            previous_node_id,
            next_node_id,
        }
    }

    pub fn set_previous_node_id(&mut self, previous_node_id: u32) {
        self.previous_node_id = Some(previous_node_id);
    }

    pub fn set_next_node_id(&mut self, next_node_id: u32) {
        self.next_node_id = Some(next_node_id);
    }
}

#[derive(Debug)]
pub struct PlayerStoryTree {
    pub past_node_choices: HashMap<u32, StoryOption>,
    pub story_path: Vec<StoryNode>,
}

impl PlayerStoryTree {
    pub fn new() -> Self {
        PlayerStoryTree {
            past_node_choices: HashMap::new(),
            story_path: Vec::new(),
        }
    }

    fn add_story_node(&mut self, story_node: StoryNode) {
        self.story_path.push(story_node);
    }

    pub fn get_last_story_node(&self) -> Option<&StoryNode> {
        self.story_path.last()
    }

    pub fn get_story_node(&self, node_id: u32) -> Option<&StoryNode> {
        self.story_path.iter().find(|node| node.id == node_id)
    }

    pub fn add_past_node_choice(&mut self, node: StoryNode, option: StoryOption) {
        self.add_story_node(node.clone());
        self.past_node_choices.insert(node.id, option);
    }

    pub fn get_past_node_choice(&self, node_id: u32) -> Option<&StoryOption> {
        self.past_node_choices.get(&node_id)
    }
}
