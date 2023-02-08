-- category-theory-for-programmers challenge 3.6.3 - 3.6.5
module Monoids where
import Control.Category

data BoolOr = BoolOr {boolOrv::Bool}

instance Semigroup BoolOr where
  (<>) a b = BoolOr {boolOrv = boolOrv a || boolOrv b}

instance Monoid BoolOr where
    mempty = BoolOr False

data BoolAnd = BoolAnd {boolAndv::Bool}

instance Semigroup BoolAnd where
  (<>) a b = BoolAnd {boolAndv = boolAndv a && boolAndv b}

instance Monoid BoolAnd where
    mempty = BoolAnd True

data MonoidArrow m a b = MonoidArrow(m -> m)
monoidArrowF :: MonoidArrow m a b -> (m -> m)
monoidArrowF (MonoidArrow(f)) = f

monoidHom :: (Monoid m) => m -> (m -> m)
monoidHom m = mappend m

instance Monoid m => Category (MonoidArrow m) where
    id = MonoidArrow(mappend mempty)
    (.) a b = MonoidArrow(monoidArrowF b Prelude.. monoidArrowF a)

instance Semigroup Int where
  (<>) a = (`mod` 3) Prelude.. (a + )

instance Monoid Int where
    mempty = 0