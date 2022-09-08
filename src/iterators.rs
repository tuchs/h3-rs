use crate::{
    constants::MAX_H3_RES,
    coord_ijk::Direction,
    h3_index::{
        H3Index, _zeroIndexDigits, isPentagon, H3_GET_INDEX_DIGIT, H3_GET_RESOLUTION,
        H3_PER_DIGIT_OFFSET, H3_SET_RESOLUTION,
    },
    H3_NULL,
};

pub struct IterCellsChildren {
    h: H3Index,
    _parentRes: i32,
    _skipDigit: i32,
}

impl IterCellsChildren {
    pub fn from_parent(mut h: H3Index, childRes: i32) -> IterCellsChildren {
        //IterCellsChildren it;

        let _parentRes = H3_GET_RESOLUTION(h);

        if childRes < _parentRes || childRes > MAX_H3_RES || h == H3_NULL {
            return Self::_null_iter();
        }

        h = _zeroIndexDigits(h, _parentRes + 1, childRes);
        H3_SET_RESOLUTION(&mut h, childRes);

        let _skipDigit: i32 = match isPentagon(h) {
            true => childRes,
            false => -1,
        };

        return IterCellsChildren {
            h,
            _parentRes,
            _skipDigit,
        };
    }

    pub fn _null_iter() -> IterCellsChildren {
        return IterCellsChildren {
            h: H3_NULL,
            _parentRes: -1,
            _skipDigit: -1,
        };
    }

    // extract the `res` digit (0--7) of the current cell
    pub fn _getResDigit(&self, res: i32) -> Direction {
        return H3_GET_INDEX_DIGIT(self.h, res);
    }

    // increment the digit (0--7) at location `res`
    // H3_PER_DIGIT_OFFSET == 3
    pub fn _incrementResDigit(&mut self, res: i32) {
        let mut val: H3Index = 1;
        val <<= H3_PER_DIGIT_OFFSET * (MAX_H3_RES - res);
        self.h += val;
    }
}

impl Iterator for IterCellsChildren {
    type Item = H3Index;

    fn next(&mut self) -> Option<Self::Item> {
        // once h == H3_NULL, the iterator returns an infinite sequence of H3_NULL
        if self.h == H3_NULL {
            return None;
        }

        let ret = self.h;

        let childRes = H3_GET_RESOLUTION(self.h);

        self._incrementResDigit(childRes);

        for i in (self._parentRes..(childRes + 1)).rev() {
            //(int i = childRes; i >= it->_parentRes; i--) {
            if i == self._parentRes {
                // if we're modifying the parent resolution digit, then we're done
                *self = IterCellsChildren::_null_iter();
                return Some(ret);
            }

            // PENTAGON_SKIPPED_DIGIT == 1
            if i == self._skipDigit && self._getResDigit(i) == Direction::PENTAGON_SKIPPED_DIGIT {
                // Then we are iterating through the children of a pentagon cell.
                // All children of a pentagon have the property that the first
                // nonzero digit between the parent and child resolutions is
                // not 1.
                // I.e., we never see a sequence like 00001.
                // Thus, we skip the `1` in this digit.
                self._incrementResDigit(i);
                self._skipDigit -= 1;
                return Some(ret);
            }

            // INVALID_DIGIT == 7
            if self._getResDigit(i) == Direction::InvalidDigit {
                self._incrementResDigit(i); // zeros out it[i] and increments it[i-1] by 1
            } else {
                break;
            }
        }
        return Some(ret);
    }
}
