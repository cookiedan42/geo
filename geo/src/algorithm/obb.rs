
use crate::{BoundingRect, Translate};
use crate::{CoordFloat, GeoNum,GeoFloat};
use crate::{Coord, MultiPoint, Point, Polygon,Rotate,LineString};

/*
https://logicatcore.github.io/scratchpad/lidar/sensor-fusion/jupyter/2021/04/20/2D-Oriented-Bounding-Box.html
*/

pub fn oriented_bounding_box<T>(points: &[Coord<T>]) -> Option<Polygon<T>>
where T: CoordFloat + GeoNum + GeoFloat {

    let l = points.len();

    // handle trivial cases 
    if l == 0 {
        return None;
    }
    else if l == 1 {
        return Some(Polygon::new(LineString::new(vec![
            points[0],
            points[0],
            points[0],
            points[0],
            ]),vec![]));
    }
    else if l == 2 {
         return Some(Polygon::new(LineString::new(vec![
            points[0],
            points[1],
            points[1],
            points[0],
            ]),vec![]));
    }
    let mp: MultiPoint<T> = MultiPoint::from_iter(points.iter().map(|x|Point::new(x.x,x.y)));

    let(meanx,meany) = mp.iter().fold((T::zero(),T::zero()),|acc,p|{
        (acc.0 + p.x(),acc.1 + p.y())
    });
    let l = T::from(l).unwrap();
    let meanx = meanx / l;
    let meany = meany / l;

    let data:Vec<[T;2]> = mp.iter().map(|p|[p.x(),p.y()]).collect();
    let cov = calculate_2d_covariance_matrix(&data);
    let evec = calculate_eigen_decomposition_col0(cov).unwrap();

    let theta_pc1 = get_eigen_angle(evec[0],evec[1]).to_degrees();

    let mp2 = mp
        .translate(-meanx,-meany)
        .rotate_around_point(-theta_pc1,Point::new(T::zero(),T::zero()));

    let bbox: Polygon<T> = mp2.bounding_rect().unwrap().into();

    let bbox: Polygon<T> = bbox.rotate_around_point(theta_pc1,Point::new(T::zero(),T::zero()));

    let bbox = bbox.translate(meanx, meany);

    Some(bbox)
}


fn get_eigen_angle<T>(v0:T,v1:T) -> T where T:CoordFloat{
    (v0/v1).atan()
}



fn calculate_2d_covariance_matrix<T>(data: &Vec<[T; 2]>) -> [[T; 2]; 2] where T: CoordFloat{

    let n = data.len();
    
    // Calculate means
    let mut mean_x = T::zero();
    let mut mean_y = T::zero();
    
    for point in data {
        mean_x = mean_x + point[0];
        mean_y = mean_y + point[1];
    }
    mean_x = mean_x / T::from(n).unwrap();
    mean_y = mean_y / T::from(n).unwrap();

    // Calculate covariances
    let mut cov_xx = T::zero();
    let mut cov_xy = T::zero();
    let mut cov_yy = T::zero();

    for point in data {
        let diff_x = point[0] - mean_x;
        let diff_y = point[1] - mean_y;
        
        cov_xx = cov_xx + diff_x * diff_x;
        cov_xy = cov_xy + diff_x * diff_y;
        cov_yy = cov_yy + diff_y * diff_y;
    }

    let n_minus_1 = T::from(n - 1).unwrap();
    let covariance = [
        [cov_xx / n_minus_1, cov_xy / n_minus_1],
        [cov_xy / n_minus_1, cov_yy / n_minus_1]
    ];

    covariance
}


fn calculate_eigen_decomposition_col0<T>(matrix: [[T; 2]; 2]) -> Option<([T; 2])> where T:CoordFloat {
    // For a 2x2 matrix [[a, b], [c, d]]
    let a = matrix[0][0];
    let b = matrix[0][1];
    let c = matrix[1][0];
    let d = matrix[1][1];

    // Calculate coefficients of characteristic equation: λ² - (a+d)λ + (ad-bc) = 0
    let trace = a + d;
    let determinant = a * d - b * c;

    // Calculate eigenvalues using quadratic formula: λ = (trace ± √(trace² - 4*determinant))/2
    let discriminant = trace * trace - T::from(4.0).unwrap() * determinant;
    
    if discriminant < T::zero() {
        return None; // Complex eigenvalues
    }

    let sqrt_discriminant = discriminant.sqrt();
    let lambda1 = (trace + sqrt_discriminant) / T::from(2.0).unwrap();

    // Calculate eigenvector col 0 only because we only use this
    // For each eigenvalue λ, solve (A - λI)v = 0

    // First eigenvector
    if b != T::zero() {
        let v1 = [b, lambda1 - a];
        let norm = (v1[0] * v1[0] + v1[1] * v1[1]).sqrt();
        Some( [v1[0] / norm, v1[1] / norm])
    } else if (a - lambda1).abs() > (c).abs() {
        Some([T::from(1.0).unwrap(),T::zero()])
    } else {
        Some([T::zero(), T::from(1.0).unwrap()])
    }

}


#[cfg(test)]
mod test {
    use super::*;
    use crate::{MultiPoint,point};
    use crate::MinimumRotatedRect;
    #[test]
    fn test() {
        let mp: MultiPoint<f64> = MultiPoint::new(vec![
            point!{x:0.0,y:0.0},
            point!{x:0.0,y:1.0},
            point!{x:1.0,y:1.0},
            ]);


        
        let bbox = oriented_bounding_box(&mp.0.iter().map(|x|x.0).collect::<Vec<Coord<f64>>>());
        let rect = mp.minimum_rotated_rect();

        assert_eq!(bbox,rect);

    }


}