

struct QueryVector {
    vec: &[f32]
}

struct PointVector {
    vec: &[f32]
    payload: &VectorPayload
}

struct VectorPayload {
    score: f32
    
    page_url: String
}

trait VectorSearchClient {
    fn search(query: QueryVector) -> Vec<&VectorPayload>;
}

fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    let mut product = 0.0;
    for i in 0..a.len() {
        product += a[i] * b[i];
    }
    return product;
}

struct simpleKNN {
    index: Vec<PointVector>
}

impl VectorSearchClient for simpleKNN {
    fn search(&self, query: QueryVector) -> Vec<&VectorPayload> {
        for vec in self.index.iter() {

        }
    }
}