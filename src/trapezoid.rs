use std::fmt::Debug;

use crate::{Vertex, VertexIndex, idx::{Idx, IdxDisplay}, nexus::Nexus, querynode::QueryNode, segment::Segment};

/// Describes a trapezoid in the trapezoidation.
#[derive(Debug)]
pub struct Trapezoid<V: Vertex, Index: VertexIndex> {
    /// The left edge constraining the trapezoid; if `None`, the trapezoid extends infinitely to the left
    left: Option<Idx<Segment<V, Index>>>,
    /// The right edge constraining the trapezoid; if `None`, the trapezoid extends infinitely to the right
    right: Option<Idx<Segment<V, Index>>>,
    /// The vertex whose y-coordinate constrains the trapezoid downward; if `None`, the trapezoid extends infinitely
    down: Option<Idx<Nexus<V, Index>>>,
    /// The vertex whose y-coordinate constrains the trapezoid upward; if `None`, the trapezoid extends infinitely
    up: Option<Idx<Nexus<V, Index>>>,
    /// The [`QueryNode`] ID of the [`Sink`](QueryNode::Sink) associated with this trapezoid
    sink: Idx<QueryNode<V, Index>>,
}

impl<V: Vertex, Index: VertexIndex> std::fmt::Display for Trapezoid<V, Index> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(up) = self.up {
            writeln!(f, "-{}-", up)?;
        }
        if let Some(left) = self.left {
            write!(f, "[{}]", left)?;
        }
        std::fmt::Display::fmt(&self.sink, f)?;
        if let Some(right) = self.right {
            write!(f, "[{}]", right)?;
        }
        writeln!(f)?;
        if let Some(down) = self.down {
            writeln!(f, "-{}-", down)?;
        }
        Ok(())
    }
}

impl<V: Vertex, Index: VertexIndex> IdxDisplay for Trapezoid<V, Index> {
    fn fmt(f: &mut std::fmt::Formatter<'_>, idx: usize) -> std::fmt::Result {
        write!(f, "t{}", idx)
    }
}

impl<V: Vertex, Index: VertexIndex> Trapezoid<V, Index> {
    /// Creates a new trapezoid unconstrained in all directions, associated with a given [`Sink`](QueryNode::Sink) node
    pub fn all(sink: Idx<QueryNode<V, Index>>) -> Self {
        Self {
            left: None,
            right: None,
            down: None,
            up: None,
            sink,
        }
    }

    /// Split the trapezoid in two using the given vertical segment, returning the newly created right trapezoid and associating both with the given [`Sink`](QueryNode::Sink) nodes.
    pub fn split_vertical(&mut self, qi_left: Idx<QueryNode<V, Index>>, qi_right: Idx<QueryNode<V, Index>>, si: Idx<Segment<V, Index>>) -> Self {
        let t_right = Self {
            left: Some(si),
            right: self.right,
            down: self.down,
            up: self.up,
            sink: qi_right,
        };

        self.right = Some(si);
        self.sink = qi_left;

        t_right
    }
    
    /// Split the trapezoid in two using the given point's y-coordinate, returning the newly created top trapezoid and associating both with the given [`Sink`](QueryNode::Sink) nodes.
    pub fn split_horizontal(&mut self, qi_down: Idx<QueryNode<V, Index>>, qi_up: Idx<QueryNode<V, Index>>, ni: Idx<Nexus<V, Index>>) -> Self {
        let t_up = Self {
            left: self.left,
            right: self.right,
            down: Some(ni),
            up: self.up,
            sink: qi_up,
        };

        self.up = Some(ni);
        self.sink = qi_down;

        t_up
    }

    /// Sets the trapezoid's lower bound
    pub fn set_down(&mut self, ni: Idx<Nexus<V, Index>>) {
        self.down = Some(ni);
    }

    /// Sets the trapezoid's left bound
    pub fn set_left(&mut self, si: Idx<Segment<V, Index>>) {
        self.left = Some(si);
    }

    /// Sets the trapezoid's right bound
    pub fn set_right(&mut self, si: Idx<Segment<V, Index>>) {
        self.right = Some(si);
    }

    /// Sets the trapezoid's sink ID
    pub fn set_sink(&mut self, qi: Idx<QueryNode<V, Index>>) {
        self.sink = qi;
    }

    /// Access the trapezoid's left bound
    pub fn left(&self) -> Option<Idx<Segment<V, Index>>> { self.left }
    /// Access the trapezoid's right bound
    pub fn right(&self) -> Option<Idx<Segment<V, Index>>> { self.right }

    /// Access the trapezoid's upper bound
    pub fn up(&self) -> Option<Idx<Nexus<V, Index>>> { self.up }
    /// Access the trapezoid's lower bound
    pub fn down(&self) -> Option<Idx<Nexus<V, Index>>> { self.down }
    
    /// Access the trapezoid's sink ID
    pub fn sink(&self) -> Idx<QueryNode<V, Index>> { self.sink }
}