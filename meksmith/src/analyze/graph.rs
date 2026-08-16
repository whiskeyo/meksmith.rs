use std::collections::{HashMap, HashSet};

use crate::analyze::SymbolTable;
use crate::diagnostic::{Diagnostic, DiagnosticCode, Emitter};
use crate::frontend::ast::{File, Item, TypeExpr};

#[derive(Debug, Clone, Default)]
pub struct DependencyGraph {
    edges: HashMap<String, HashSet<String>>,
}

impl DependencyGraph {
    pub fn build(file: &File, symbols: &SymbolTable, _emitter: &mut Emitter) -> Self {
        let mut graph = Self::default();

        for item in &file.items {
            let name = item.node.name().to_string();
            let deps = dependencies_for_item(&item.node);
            for dep in deps {
                if symbols.get(&dep).is_some() || is_primitive(&dep) {
                    graph.edges.entry(name.clone()).or_default().insert(dep);
                }
            }
        }

        graph
    }

    pub fn topological_order(&self, emitter: &mut Emitter) -> Vec<String> {
        let mut nodes: HashSet<String> = self.edges.keys().cloned().collect();
        for deps in self.edges.values() {
            nodes.extend(deps.iter().cloned());
        }

        let mut in_degree: HashMap<String, usize> =
            nodes.iter().map(|name| (name.clone(), 0)).collect();
        let mut dependents: HashMap<String, Vec<String>> = HashMap::new();

        for (user, deps) in &self.edges {
            for dep in deps {
                *in_degree.entry(user.clone()).or_default() += 1;
                dependents
                    .entry(dep.clone())
                    .or_default()
                    .push(user.clone());
            }
        }

        let mut ready: Vec<String> = in_degree
            .iter()
            .filter(|(_, degree)| **degree == 0)
            .map(|(name, _)| name.clone())
            .collect();
        ready.sort();

        let mut order = Vec::new();
        while let Some(node) = ready.pop() {
            order.push(node.clone());
            if let Some(users) = dependents.get(&node) {
                for user in users {
                    if let Some(degree) = in_degree.get_mut(user) {
                        *degree -= 1;
                        if *degree == 0 {
                            ready.push(user.clone());
                        }
                    }
                }
            }
            ready.sort();
        }

        if order.len() != nodes.len() {
            emitter.emit(Diagnostic::error(
                DiagnosticCode::S0007CircularType,
                crate::frontend::span::SimpleSpan::new(0, 0),
                "circular type dependency detected",
            ));
            order.extend(
                nodes
                    .into_iter()
                    .filter(|name| !order.contains(name))
                    .collect::<Vec<_>>(),
            );
            order.sort();
            order.dedup();
        }

        order
    }

    pub fn dependencies_of(&self, name: &str) -> Option<&HashSet<String>> {
        self.edges.get(name)
    }
}

fn dependencies_for_item(item: &Item) -> HashSet<String> {
    let mut deps = HashSet::new();
    match item {
        Item::Structure(s) => {
            for field in &s.fields {
                collect_type_deps(&field.node.ty, &mut deps);
            }
            for param in &s.params {
                if !is_primitive(&param.ty) && !is_generic(&param.ty) {
                    deps.insert(param.ty.clone());
                }
            }
        }
        Item::Enumerated(_) => {}
        Item::Choice(c) => {
            for arm in &c.arms {
                collect_type_deps(&arm.node.ty, &mut deps);
            }
            for param in &c.params {
                if !is_primitive(&param.ty) && !is_generic(&param.ty) {
                    deps.insert(param.ty.clone());
                }
            }
            if !is_primitive(&c.discriminant) && !is_generic(&c.discriminant) {
                deps.insert(c.discriminant.clone());
            }
        }
        Item::TypeAlias(t) => {
            collect_type_deps(&t.ty, &mut deps);
        }
    }
    deps
}

fn collect_type_deps(ty: &TypeExpr, deps: &mut HashSet<String>) {
    match ty {
        TypeExpr::Null => {}
        TypeExpr::Named {
            name,
            generics,
            args,
        } => {
            if !is_primitive(name) && !is_generic(name) {
                deps.insert(name.clone());
            }
            for expr in generics.iter().chain(args.iter()) {
                collect_expr_type_deps(expr, deps);
            }
        }
    }
}

fn collect_expr_type_deps(expr: &crate::frontend::ast::Expr, deps: &mut HashSet<String>) {
    match expr {
        crate::frontend::ast::Expr::Ident(name) => {
            if !is_primitive(name) && !is_generic(name) {
                deps.insert(name.clone());
            }
        }
        crate::frontend::ast::Expr::Field { base, .. } => collect_expr_type_deps(base, deps),
        crate::frontend::ast::Expr::Call { callee, args } => {
            collect_expr_type_deps(callee, deps);
            for arg in args {
                collect_expr_type_deps(arg, deps);
            }
        }
        crate::frontend::ast::Expr::Binary { lhs, rhs, .. } => {
            collect_expr_type_deps(lhs, deps);
            collect_expr_type_deps(rhs, deps);
        }
        crate::frontend::ast::Expr::Literal(_) => {}
    }
}

fn is_primitive(name: &str) -> bool {
    matches!(
        name,
        "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "boolean" | "byte" | "null"
    )
}

fn is_generic(name: &str) -> bool {
    matches!(
        name,
        "DynamicArray" | "GreedyArray" | "StaticArray" | "Optional" | "BitSlice" | "CountedArray"
    )
}
