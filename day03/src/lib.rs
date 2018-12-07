use std::str::FromStr;

#[derive(Debug)]
pub struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Debug)]
pub struct ParseClaimError;

impl FromStr for Claim {
    type Err = ParseClaimError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let id = parts.next().ok_or(ParseClaimError)?[1..]
            .parse()
            .map_err(|_| ParseClaimError)?;

        parts.next();

        let mut coords = parts
            .next()
            .ok_or(ParseClaimError)?
            .split(|c: char| !c.is_numeric());

        let x = coords
            .next()
            .ok_or(ParseClaimError)?
            .parse()
            .map_err(|_| ParseClaimError)?;

        let y = coords
            .next()
            .ok_or(ParseClaimError)?
            .parse()
            .map_err(|_| ParseClaimError)?;

        let mut bounds = parts
            .next()
            .ok_or(ParseClaimError)?
            .split(|c: char| !c.is_numeric());

        let w = bounds
            .next()
            .ok_or(ParseClaimError)?
            .parse()
            .map_err(|_| ParseClaimError)?;

        let h = bounds
            .next()
            .ok_or(ParseClaimError)?
            .parse()
            .map_err(|_| ParseClaimError)?;

        Ok(Claim { id, x, y, w, h })
    }
}

pub struct Fabric {
    w: u32,
    covered: Vec<u32>,
}

impl Fabric {
    pub fn new(claims: &[Claim]) -> Self {
        let w = claims
            .iter()
            .map(|claim| claim.x + claim.w)
            .max()
            .unwrap_or(0);

        let h = claims
            .iter()
            .map(|claim| claim.y + claim.h)
            .max()
            .unwrap_or(0);

        let mut covered = vec![0; (w * h) as usize];

        for claim in claims {
            for y in claim.y..(claim.y + claim.h) {
                for x in claim.x..(claim.x + claim.w) {
                    covered[(y * w + x) as usize] += 1;
                }
            }
        }

        Fabric { w, covered }
    }

    pub fn covered_area(&self) -> usize {
        self.covered.iter().filter(|&&i| i > 1).count()
    }

    pub fn intact_claim_id(&self, claims: &[Claim]) -> Option<u32> {
        'claims: for claim in claims {
            for y in claim.y..(claim.y + claim.h) {
                for x in claim.x..(claim.x + claim.w) {
                    if self.covered[(y * self.w + x) as usize] > 1 {
                        continue 'claims;
                    }
                }
            }

            return Some(claim.id);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_covered_area() {
        let claims = vec![
            "#1 @ 1,3: 4x4".parse().unwrap(),
            "#2 @ 3,1: 4x4".parse().unwrap(),
            "#3 @ 5,5: 2x2".parse().unwrap(),
        ];

        let fabric = Fabric::new(&claims);

        assert_eq!(4, fabric.covered_area());
    }

    #[test]
    fn sample_intact_claim_id() {
        let claims = vec![
            "#1 @ 1,3: 4x4".parse().unwrap(),
            "#2 @ 3,1: 4x4".parse().unwrap(),
            "#3 @ 5,5: 2x2".parse().unwrap(),
        ];

        let fabric = Fabric::new(&claims);

        assert_eq!(Some(3), fabric.intact_claim_id(&claims));
    }
}
