

struct QueryVector {
    vec: [f32],
}

struct PointVector {
    vec: [f32],
    payload: VectorPayload,
}

struct VectorPayload {
    page_url: String
}

struct VectorSearchResult {
    payload: &VectorPayload,
    score: f32, 
}


trait VectorSearchClient {
    fn search(&self, query: QueryVector) -> Vec<&VectorSearchResult;
    fn point_count(&self) -> i32; 
}









struct simpleKNN {
    /*
    
        This is a simple vector search client. It will store all of the PointVectors in a single Vec, and then in order to search, it will find the dot product of the QueryVector with each one of the PointVectors, sort the results, and then return the highest of the sorted results. 
    
    */
    index: Vec<PointVector>
}

fn sortVectorSearchResults(results: &mut Vec<&VectorSearchResult>) {
    /*
    
        Given a vec containing references to search results, sort them in descending order
    
    */

    let len = results.len()

    for i in 0..len {
        let mut max_idx = i;
        for j in (i+1)..len {
            if results[j] > results[max_idx] {
                max_idx = j; 
            }
        }
        let tmp = results[max_idx];
        results[max_idx] = results[i];
        results[i] = tmp;
    }


}

impl VectorSearchClient for simpleKNN {
    fn search(&self, query: QueryVector) -> Vec<&VectorSearchResult> {
       
    }
    fn point_count(&self) -> i32 {
        return self.index.len();
    }
}


fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    let mut product = 0.0;
    for i in 0..a.len() {
        product += a[i] * b[i];
    }
    return product;
}