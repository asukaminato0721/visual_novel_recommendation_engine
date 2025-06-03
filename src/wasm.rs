use crate::recommender::VisualNovelRecommender;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, visual-novel-recommendation-engine!");
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Serialize, Deserialize)]
pub struct RecommendationResult {
    pub tag_recommendations: Vec<RecommendationItem>,
    pub user_recommendations: Vec<RecommendationItem>,
    pub combined_recommendations: Vec<RecommendationItem>,
}

#[derive(Serialize, Deserialize)]
pub struct RecommendationItem {
    pub id: i32,
    pub title: String,
    pub url: String,
}

#[wasm_bindgen]
pub struct WasmRecommender {
    recommender: VisualNovelRecommender,
}

#[wasm_bindgen]
impl WasmRecommender {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmRecommender {
        console_error_panic_hook::set_once();

        let ignore_tags = vec![32, 2040, 2461, 1434, 1431, 43];

        let recommender = VisualNovelRecommender::new(
            25,  // num_recommendations
            1.5, // tag_weight
            1.0, // vote_weight
            2.0, // tag_exp
            1.0, // vote_exp
            ignore_tags,
            false, // verbose
            false, // skip_recs
        );

        WasmRecommender { recommender }
    }

    #[wasm_bindgen]
    pub fn get_recommendations(&self, vn_id: i32) -> JsValue {
        console_log!("Getting recommendations for VN ID: {}", vn_id);

        let tag_recs = self.recommender.get_tag_recommendations(vn_id);
        let user_recs = self.recommender.get_user_recommendations(vn_id);
        let combined_recs = self.recommender.get_combined_recommendations(vn_id);

        let result = RecommendationResult {
            tag_recommendations: tag_recs
                .iter()
                .map(|&id| RecommendationItem {
                    id,
                    title: self.recommender.get_title(id).to_string(),
                    url: format!("https://vndb.org/v{}", id),
                })
                .collect(),
            user_recommendations: user_recs
                .iter()
                .map(|&id| RecommendationItem {
                    id,
                    title: self.recommender.get_title(id).to_string(),
                    url: format!("https://vndb.org/v{}", id),
                })
                .collect(),
            combined_recommendations: combined_recs
                .iter()
                .map(|&id| RecommendationItem {
                    id,
                    title: self.recommender.get_title(id).to_string(),
                    url: format!("https://vndb.org/v{}", id),
                })
                .collect(),
        };

        serde_wasm_bindgen::to_value(&result).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_title(&self, vn_id: i32) -> String {
        self.recommender.get_title(vn_id).to_string()
    }

    #[wasm_bindgen]
    pub fn load_vn_titles(&mut self, data: &str) -> bool {
        console_log!("Loading VN titles...");

        match self.recommender.load_vn_titles_from_string(data) {
            Ok(_) => {
                console_log!("VN titles loaded successfully");
                true
            }
            Err(e) => {
                console_log!("Error loading VN titles: {}", e);
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn load_votes(&mut self, data: &str) -> bool {
        console_log!("Loading votes data...");

        match self.recommender.load_ratings_from_string(data) {
            Ok(_) => {
                console_log!("Votes data loaded successfully");
                true
            }
            Err(e) => {
                console_log!("Error loading votes: {}", e);
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn load_tags(&mut self, data: &str) -> bool {
        console_log!("Loading tags data...");

        match self.recommender.load_tags_from_string(data) {
            Ok(_) => {
                console_log!("Tags data loaded successfully");
                true
            }
            Err(e) => {
                console_log!("Error loading tags: {}", e);
                false
            }
        }
    }

    // Keep the old method for backwards compatibility
    #[wasm_bindgen]
    pub fn process_csv_data(&mut self, csv_data: &str) -> bool {
        console_log!("Processing generic CSV data...");
        // For now, assume it's votes data if called
        self.load_votes(csv_data)
    }
}
