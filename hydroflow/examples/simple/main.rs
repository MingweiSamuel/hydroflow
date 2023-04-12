use clap::{Parser, ValueEnum};
use hydroflow::hydroflow_syntax;

#[derive(Parser, Debug, Clone, ValueEnum)]
enum GraphType {
    Mermaid,
    Dot,
    Json,
}
#[derive(Parser, Debug)]
struct Opts {
    #[clap(value_enum, long)]
    graph: Option<GraphType>,
}

#[tokio::main]
pub async fn main() {
    let opts = Opts::parse();

    let mut df = hydroflow_syntax! {
        repeat_iter([0]) -> for_each(|x| println!("{} {}", context.current_tick(), x));
    };

    if let Some(graph) = opts.graph {
        let serde_graph = df
            .meta_graph()
            .expect("No graph found, maybe failed to parse.");
        match graph {
            GraphType::Mermaid => {
                println!("{}", serde_graph.to_mermaid());
            }
            GraphType::Dot => {
                println!("{}", serde_graph.to_dot())
            }
            GraphType::Json => {
                unimplemented!();
                // println!("{}", serde_graph.to_json())
            }
        }
    }

    df.run_async().await;
}
