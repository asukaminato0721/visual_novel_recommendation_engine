// src/recommender.rs
use crate::data::{Rating, Tag, VnTitle};
use csv::ReaderBuilder;
use sprs::{CsMat, TriMat};
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    sync::Arc,
};

pub struct VisualNovelRecommender {
    pub num_vns: usize,
    pub tag_weight: f64,
    pub vote_weight: f64,
    pub tag_exp: f64,
    pub vote_exp: f64,
    pub ignore_tags: Vec<i32>,
    pub verbose: bool,
    pub skip_recs: bool,
    // Data structures
    pub vn_titles: Vec<VnTitle>,
    pub ratings: Vec<Rating>,
    pub average_ratings: HashMap<i32, f64>,
    pub tags: Vec<Tag>,
    pub similarity_matrix: Option<CsMat<f64>>,
}

impl VisualNovelRecommender {
    // Constructor and load_data are implemented as shown in previous response
    pub fn new(
        num_vns: usize,
        tag_weight: f64,
        vote_weight: f64,
        tag_exp: f64,
        vote_exp: f64,
        ignore_tags: Vec<i32>,
        verbose: bool,
        skip_recs: bool,
    ) -> Self {
        let mut recommender = Self {
            num_vns,
            tag_weight,
            vote_weight,
            tag_exp,
            vote_exp,
            ignore_tags,
            verbose,
            skip_recs,
            vn_titles: Vec::new(),
            ratings: Vec::new(),
            average_ratings: HashMap::new(),
            tags: Vec::new(),
            similarity_matrix: None,
        };

        recommender.load_data().unwrap();
        recommender
    }
    pub fn load_data(&mut self) -> Result<(), Box<dyn Error>> {
        // Load titles
        if self.verbose {
            println!("Loading titles");
        }

        let file = File::open("data/vn_titles")?;
        let reader = BufReader::new(file);
        let mut vn_titles = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 5 {
                let vn_id_str = parts[0].trim_start_matches('v');
                let vn_id = match vn_id_str.parse::<i32>() {
                    Ok(id) => id,
                    Err(_) => continue, // Skip invalid IDs
                };

                let language = parts[1].to_string();
                let official = parts[2] == "t";
                let title = parts[3].to_string();
                let latin_title = if parts[4] == "\\N" {
                    None
                } else {
                    Some(parts[4].to_string())
                };

                vn_titles.push(VnTitle {
                    vn_id,
                    language: language.into(),
                    official,
                    title: title.into(),
                    latin_title: latin_title.map(|x| x.into()),
                });
            }
        }

        self.vn_titles = vn_titles;

        if self.skip_recs {
            return Ok(());
        }

        // Load ratings
        if self.verbose {
            println!("Loading votes");
        }

        let file = File::open("data/votes")?;
        let reader = BufReader::new(file);
        let mut ratings = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let vn_id = match parts[0].parse::<i32>() {
                    Ok(id) => id,
                    Err(_) => continue,
                };

                let user_id = match parts[1].parse::<i32>() {
                    Ok(id) => id,
                    Err(_) => continue,
                };

                let raw_rating = match parts[2].parse::<f64>() {
                    Ok(r) => r,
                    Err(_) => continue,
                };

                // Apply the vote exponential transformation
                let rating = raw_rating.signum() * raw_rating.abs().powf(self.vote_exp);

                let date = parts[3].to_string();

                ratings.push(Rating {
                    vn_id,
                    user_id,
                    rating,
                    date: date.into(),
                });
            }
        }

        self.ratings = ratings;

        // Calculate average ratings
        if self.verbose {
            println!("Calculating average ratings");
        }

        let mut rating_sums: HashMap<i32, f64> = HashMap::new();
        let mut rating_counts: HashMap<i32, i32> = HashMap::new();

        for rating in &self.ratings {
            *rating_sums.entry(rating.vn_id).or_insert(0.0) += rating.rating;
            *rating_counts.entry(rating.vn_id).or_insert(0) += 1;
        }

        self.average_ratings = rating_sums
            .iter()
            .map(|(vn_id, sum)| {
                let count = *rating_counts.get(vn_id).unwrap_or(&1) as f64;
                (*vn_id, sum / count)
            })
            .collect();

        // Load tag data
        if self.verbose {
            println!("Loading tags_vn");
        }

        let mut reader = ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_path("data/tags_vn")?;

        let mut tags = Vec::new();

        while let Some(result) = reader.records().next() {
            let record = result?;
            if record.len() >= 5 {
                let tag_id_str = &record[1][1..];
                let tag_id = match tag_id_str.parse::<i32>() {
                    Ok(id) => id,
                    Err(_) => continue,
                };

                let vn_id_str = &record[2][1..];
                let vn_id = match vn_id_str.parse::<i32>() {
                    Ok(id) => id,
                    Err(_) => continue,
                };

                let raw_rating = match record[4].parse::<f64>() {
                    Ok(r) => r,
                    Err(_) => continue,
                };

                // Skip ignored tags
                if self.ignore_tags.contains(&tag_id) {
                    continue;
                }

                // Apply tag exponential transformation
                let rating = raw_rating.signum() * raw_rating.abs().powf(self.tag_exp);

                tags.push(Tag {
                    tag_id,
                    vn_id,
                    rating,
                });
            }
        }

        self.tags = tags;

        // Calculate average vote for each tag for each VN
        if self.verbose {
            println!("Building average tags");
        }

        let mut tag_sums: HashMap<(i32, i32), f64> = HashMap::new();
        let mut tag_counts: HashMap<(i32, i32), i32> = HashMap::new();

        for tag in &self.tags {
            let key = (tag.vn_id, tag.tag_id);
            *tag_sums.entry(key).or_insert(0.0) += tag.rating;
            *tag_counts.entry(key).or_insert(0) += 1;
        }

        let average_tag_votes: Vec<(i32, i32, f64)> = tag_sums
            .iter()
            .map(|((vn_id, tag_id), sum)| {
                let count = *tag_counts.get(&(*vn_id, *tag_id)).unwrap_or(&1) as f64;
                let avg = sum / count;
                (*vn_id, *tag_id, avg)
            })
            .filter(|(_, _, rating)| *rating != 0.0) // Filter out zero ratings
            .collect();

        // Create a sparse matrix for VN x tag
        if self.verbose {
            println!("Calculating tag similarity matrix.");
        }

        // Find max VN and tag IDs to size the matrix
        let max_vn_id = average_tag_votes
            .iter()
            .map(|(vn_id, _, _)| *vn_id)
            .max()
            .unwrap_or(0) as usize;
        let max_tag_id = average_tag_votes
            .iter()
            .map(|(_, tag_id, _)| *tag_id)
            .max()
            .unwrap_or(0) as usize;

        // Create triplet matrix
        let mut triplet_matrix = TriMat::new((max_vn_id + 1, max_tag_id + 1));

        // Fill the matrix
        for (vn_id, tag_id, rating) in average_tag_votes {
            triplet_matrix.add_triplet(vn_id as usize, tag_id as usize, rating);
        }

        // Convert to CSR format
        let data_sparse = triplet_matrix.to_csr();

        self.similarity_matrix = Some(data_sparse);

        if self.verbose {
            println!("Similarity matrix computed.");
            println!("Loading complete.");
            println!();
        }

        Ok(())
    }
    pub fn get_average_rating(&self, vn_id: i32) -> Result<f64, String> {
        match self.average_ratings.get(&vn_id) {
            Some(rating) => Ok(*rating),
            None => Err("No ratings available for this VN.".to_string()),
        }
    }

    pub fn get_last_vn_id(&self) -> Option<i32> {
        self.vn_titles.iter().map(|vn| vn.vn_id).max()
    }

    pub fn get_title(&self, vn_id: i32) -> Arc<str> {
        // Get titles for this VN ID
        let vn_data: Vec<&VnTitle> = self
            .vn_titles
            .iter()
            .filter(|vn| vn.vn_id == vn_id)
            .collect();

        if vn_data.is_empty() {
            return format!("v{}", vn_id).into();
        }

        // Check for English title
        if let Some(en_title) = vn_data
            .iter()
            .find(|vn| vn.language == "en".into())
            .map(|vn| &vn.title)
        {
            return en_title.clone();
        }

        // Check for Latin Japanese title
        if let Some(jp_latin_title) = vn_data
            .iter()
            .find(|vn| vn.language == "ja".into() && vn.latin_title.is_some())
            .and_then(|vn| vn.latin_title.as_ref())
        {
            if jp_latin_title != &"\\N".into() {
                return jp_latin_title.clone();
            }
        }

        // Check for Japanese title
        if let Some(jp_title) = vn_data
            .iter()
            .find(|vn| vn.language == "ja".into())
            .map(|vn| &vn.title)
        {
            return jp_title.clone();
        }

        // Check for official Latin title
        if let Some(official_latin_title) = vn_data
            .iter()
            .find(|vn| vn.official && vn.latin_title.is_some())
            .and_then(|vn| vn.latin_title.as_ref())
        {
            if official_latin_title != &"\\N".into() {
                return official_latin_title.clone();
            }
        }

        // Check for official title
        if let Some(official_title) = vn_data.iter().find(|vn| vn.official).map(|vn| &vn.title) {
            return official_title.clone();
        }

        format!("v{}", vn_id).into()
    }

    // Helper function for min-max normalization
    fn min_max_normalize(&self, scores: &mut HashMap<i32, f64>) {
        if scores.is_empty() {
            return;
        }

        // Find min and max values
        let values: Vec<f64> = scores.values().cloned().collect();
        let min_val = values
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&0.0);
        let max_val = values
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(&1.0);

        // Avoid division by zero
        let range = max_val - min_val;
        if range.abs() < f64::EPSILON {
            // If all values are the same, set them all to 1.0
            for val in scores.values_mut() {
                *val = 1.0;
            }
            return;
        }

        // Apply normalization
        for val in scores.values_mut() {
            *val = (*val - *min_val) / range;
        }
    }

    pub fn get_user_recommendations_scores(&self, vn_id: i32) -> HashMap<i32, f64> {
        // Find users who rated this VN
        let users_who_rated: Vec<i32> = self
            .ratings
            .iter()
            .filter(|rating| rating.vn_id == vn_id)
            .map(|rating| rating.user_id)
            .collect();

        // Find VNs these users rated
        let mut similar_vns: HashMap<i32, (f64, usize)> = HashMap::new();

        for rating in self.ratings.iter() {
            if users_who_rated.contains(&rating.user_id) && rating.vn_id != vn_id {
                let entry = similar_vns.entry(rating.vn_id).or_insert((0.0, 0));
                entry.0 += rating.rating;
                entry.1 += 1;
            }
        }

        // Calculate average and scores
        let scores: HashMap<i32, f64> = similar_vns
            .iter()
            .map(|(vn_id, (total_rating, count))| {
                let avg_rating = total_rating / *count as f64;
                (*vn_id, avg_rating * *count as f64)
            })
            .collect();

        // Get top N VNs by score
        let mut score_vec: Vec<(i32, f64)> = scores.into_iter().collect();
        score_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        score_vec.truncate(self.num_vns);

        // Convert to HashMap with VN IDs as keys
        let mut scores: HashMap<i32, f64> = score_vec.clone().into_iter().collect();

        // Apply min-max normalization
        self.min_max_normalize(&mut scores);

        scores
    }

    pub fn get_tag_recommendations_score(&self, vn_id: i32) -> HashMap<i32, f64> {
        // Check if we have a data matrix
        let data_matrix = match &self.similarity_matrix {
            Some(matrix) => matrix,
            None => return HashMap::new(),
        };

        // Find the row corresponding to the given VN
        let row_idx = vn_id as usize;
        if row_idx >= data_matrix.rows() {
            return HashMap::new();
        }

        // Get the row for this VN
        let vn_row = match data_matrix.outer_view(row_idx) {
            Some(row) => row,
            None => return HashMap::new(),
        };

        // Calculate cosine similarities with other VNs on-demand
        let mut similarities = Vec::new();

        // Get the magnitude of the current VN vector for cosine similarity
        let vn_magnitude = (vn_row.iter().map(|(_, val)| val * val).sum::<f64>()).sqrt();
        if vn_magnitude == 0.0 {
            return HashMap::new();
        }

        // Calculate similarities only for a subset of VNs (e.g., 10000 most popular)
        let top_vns: Vec<usize> = self
            .tags
            .iter()
            .map(|t| t.vn_id as usize)
            .filter(|&id| id != row_idx && id < data_matrix.rows())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        for other_id in top_vns {
            if let Some(other_row) = data_matrix.outer_view(other_id) {
                let other_magnitude =
                    (other_row.iter().map(|(_, val)| val * val).sum::<f64>()).sqrt();
                if other_magnitude == 0.0 {
                    continue;
                }

                // Calculate dot product
                let mut dot_product = 0.0;
                for (idx1, val1) in vn_row.iter() {
                    for (idx2, val2) in other_row.iter() {
                        if idx1 == idx2 {
                            dot_product += val1 * val2;
                        }
                    }
                }

                let similarity = dot_product / (vn_magnitude * other_magnitude);
                similarities.push((other_id, similarity));
            }
        }

        // Sort by similarity score in descending order
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Take top N items
        similarities.truncate(self.num_vns);

        // Convert to HashMap with VN IDs as keys
        let mut scores: HashMap<i32, f64> = similarities
            .into_iter()
            .map(|(idx, score)| (idx as i32, score))
            .collect();

        // Apply min-max normalization
        self.min_max_normalize(&mut scores);

        scores
    }

    pub fn get_combined_recommendations_score(&self, vn_id: i32) -> HashMap<i32, f64> {
        // Get recommendations from both models
        let user_recs = self.get_user_recommendations_scores(vn_id);
        let tag_recs = self.get_tag_recommendations_score(vn_id);

        // Combine scores with weights
        let mut combined_scores: HashMap<i32, f64> = HashMap::new();

        // Add user-based recommendations
        for (id, score) in user_recs {
            *combined_scores.entry(id).or_insert(0.0) += score * self.vote_weight;
        }

        // Add tag-based recommendations
        for (id, score) in tag_recs {
            *combined_scores.entry(id).or_insert(0.0) += score * self.tag_weight;
        }

        // Sort and take top N
        let mut score_vec: Vec<(i32, f64)> = combined_scores.into_iter().collect();
        score_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        score_vec.truncate(self.num_vns);

        score_vec.into_iter().collect()
    }

    pub fn resize_list(&self, mut list: Vec<i32>) -> Vec<i32> {
        // Pad with zeros if too short
        if list.len() < self.num_vns {
            list.resize(self.num_vns, 0);
        }

        // Truncate if too long
        if list.len() > self.num_vns {
            list.truncate(self.num_vns);
        }

        list
    }

    pub fn get_user_recommendations(&self, vn_id: i32) -> Vec<i32> {
        let scores = self.get_user_recommendations_scores(vn_id);
        let mut score_vec: Vec<(i32, f64)> = scores.into_iter().collect();
        score_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let ids: Vec<i32> = score_vec.into_iter().map(|(id, _)| id).collect();
        self.resize_list(ids)
    }

    pub fn get_tag_recommendations(&self, vn_id: i32) -> Vec<i32> {
        let scores = self.get_tag_recommendations_score(vn_id);
        let mut score_vec: Vec<(i32, f64)> = scores.into_iter().collect();
        score_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let ids: Vec<i32> = score_vec.into_iter().map(|(id, _)| id).collect();
        self.resize_list(ids)
    }

    pub fn get_combined_recommendations(&self, vn_id: i32) -> Vec<i32> {
        let scores = self.get_combined_recommendations_score(vn_id);
        let mut score_vec: Vec<(i32, f64)> = scores.into_iter().collect();
        score_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let ids: Vec<i32> = score_vec.into_iter().map(|(id, _)| id).collect();
        self.resize_list(ids)
    }
}
