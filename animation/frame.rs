#[derive(Clone)]
pub struct Frame<const N: usize> {
    pub m_value: [f32; N],
    pub m_in: [f32; N],
    pub m_out: [f32; N],
    pub time: f32,
}

#[allow(unused)]
pub type ScalarFrame = Frame<1>;
pub type VectorFrame = Frame<3>;
pub type QuaternionFrame = Frame<4>;

impl<const N: usize> Frame<N> {
    pub const ONE: Self = Self {
        time: 0.0,
        m_value: [1.0; N],
        m_in: [0.0; N],
        m_out: [0.0; N],
    };

    pub fn new() -> Self {
        Self {
            time: 0.0,
            m_value: [0.0; N],
            m_in: [0.0; N],
            m_out: [0.0; N],
        }
    }
}
