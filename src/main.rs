use clap::Parser;
use serde::Serialize;
use visual_novel_recommendation_engine::recommender::VisualNovelRecommender;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    vn_id: i32,

    #[arg(short, long, default_value_t = 25)]
    num_recommendations: usize,

    #[arg(long, default_value_t = 1.5)]
    tag_weight: f64,

    #[arg(long, default_value_t = 1.0)]
    vote_weight: f64,
    
    #[arg(long, default_value_t = false)]
    json: bool,
}

#[derive(Serialize)]
struct RecommendationItem {
    id: i32,
    title: String,
    rank: usize,
}

#[derive(Serialize)]
struct RecommendationResult {
    source_vn: RecommendationItem,
    tag_recommendations: Vec<RecommendationItem>,
    vote_recommendations: Vec<RecommendationItem>,
    combined_recommendations: Vec<RecommendationItem>,
}

fn main() {
    let args = Args::parse();

    if args.vn_id <= 0 {
        println!("Please provide a valid VN ID with --vn-id");
        return;
    }

    // Default ignored tags
    let ignore_tags = vec![32, 2040, 2461, 1434, 1431, 43];

    // Initialize recommender
    let recommender = VisualNovelRecommender::new(
        args.num_recommendations,
        args.tag_weight,
        args.vote_weight,
        2.0, // tag_exp
        1.0, // vote_exp
        ignore_tags,
        !args.json,  // only be verbose if not outputting JSON
        false, // skip_recs
    );

    // Get recommendations
    let combined_recommendations = recommender.get_combined_recommendations(args.vn_id);
    let tag_recommendations = recommender.get_tag_recommendations(args.vn_id);
    let user_recommendations = recommender.get_user_recommendations(args.vn_id);
    
    let source_title = recommender.get_title(args.vn_id).to_string();

    if args.json {
        // Create JSON output
        let result = RecommendationResult {
            source_vn: RecommendationItem {
                id: args.vn_id,
                title: source_title,
                rank: 0,
            },
            tag_recommendations: tag_recommendations
                .iter()
                .enumerate()
                .map(|(i, vn_id)| RecommendationItem {
                    id: *vn_id,
                    title: recommender.get_title(*vn_id).to_string(),
                    rank: i + 1,
                })
                .collect(),
            vote_recommendations: user_recommendations
                .iter()
                .enumerate()
                .map(|(i, vn_id)| RecommendationItem {
                    id: *vn_id,
                    title: recommender.get_title(*vn_id).to_string(),
                    rank: i + 1,
                })
                .collect(),
            combined_recommendations: combined_recommendations
                .iter()
                .enumerate()
                .map(|(i, vn_id)| RecommendationItem {
                    id: *vn_id,
                    title: recommender.get_title(*vn_id).to_string(),
                    rank: i + 1,
                })
                .collect(),
        };

        // Print JSON output
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
    } else {
        // Display text results
        println!(
            "Recommendations for {}: {}",
            args.vn_id,
            source_title
        );
        println!("--------------------------------------------------");
        println!("Tag Recommendations:");
        for (i, vn_id) in tag_recommendations.iter().enumerate() {
            println!(
                "{}. {} (ID: {})",
                i + 1,
                recommender.get_title(*vn_id),
                vn_id
            );
        }
        println!("--------------------------------------------------");
        println!("Vote Recommendations:");
        for (i, vn_id) in user_recommendations.iter().enumerate() {
            println!(
                "{}. {} (ID: {})",
                i + 1,
                recommender.get_title(*vn_id),
                vn_id
            );
        }
        println!("--------------------------------------------------");
        println!("Combined Recommendations:");

        for (i, vn_id) in combined_recommendations.iter().enumerate() {
            println!(
                "{}. {} (ID: {})",
                i + 1,
                recommender.get_title(*vn_id),
                vn_id
            );
        }
    }
}
