use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use colored::Colorize;
use pathdiff::diff_paths;

use crate::compiler::Context;
use crate::plugin::Plugin;

pub struct LoopDetector {}

impl Plugin for LoopDetector {
    fn name(&self) -> &str {
        "loop_detector"
    }

    fn generate_begin(&self, context: &Arc<Context>) -> Result<()> {
        if let Some(detect_loop) = &context.config.experimental.detect_loop
            && !context.args.watch
        {
            let module_graph = context.module_graph.read().unwrap();
            let (_, loops) = module_graph.toposort();

            let loop_lines = loops
                .iter()
                .filter(|ids| {
                    if detect_loop.ignore_node_modules {
                        !ids.iter().any(|id| id.id.contains("node_modules"))
                    } else {
                        true
                    }
                })
                .map(|module_ids| {
                    let loop_end = module_ids.first().unwrap().clone();

                    module_ids
                        .iter()
                        .chain(std::iter::once(&loop_end))
                        .map(|id| {
                            let absolute_path = PathBuf::from(id.id.clone());
                            let relative_path =
                                diff_paths(&absolute_path, &context.root).unwrap_or(absolute_path);
                            let relative_path = relative_path.to_string_lossy().to_string();

                            format!(r#""{}""#, relative_path)
                        })
                        .collect::<Vec<_>>()
                        .join(" -> ")
                })
                .collect::<Vec<_>>();

            if !loop_lines.is_empty() {
                for l in &loop_lines {
                    println!("{} Found a Dependency Loop: {}", "Warning".yellow(), l);
                }

                if detect_loop.graphviz {
                    let dot_content = loop_lines.join("\n");
                    let dot = format!(r#"digraph Loop {{\n{}\n}}"#, dot_content);
                    std::fs::write(context.root.join("_mako_loop_detector.dot"), dot)?;
                }
            }

            Ok(())
        } else {
            Ok(())
        }
    }
}