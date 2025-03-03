use clap::Parser;
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
        true,  // verbose
        false, // skip_recs
    );

    // Get recommendations
    let combined_recommendations = recommender.get_combined_recommendations(args.vn_id);
    let tag_recommendations = recommender.get_tag_recommendations(args.vn_id);
    let user_recommendations = recommender.get_user_recommendations(args.vn_id);

    // Display results
    println!(
        "Recommendations for {}: {}",
        args.vn_id,
        recommender.get_title(args.vn_id)
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
