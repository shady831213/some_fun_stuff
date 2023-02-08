-- category-theory-for-programmers challenge 3.6SingleNodeCat
module FreeCat where
import Control.Category

data SingleNodeCatObj a = SingleNodeCatObj

instance Functor SingleNodeCatObj where
    fmap _ _ = SingleNodeCatObj

data SingleNodeCatArrow a b = SingleNodeCatId
singleNodeCatEval :: a -> SingleNodeCatArrow a b -> SingleNodeCatObj b
singleNodeCatEval _ _ = fmap Prelude.id SingleNodeCatObj

instance Category SingleNodeCatArrow where
    id = SingleNodeCatId
    (.) _ _ = undefined



data SingleNodeSingleArrowCatArrow a b = SingleNodeSingleArrowCatId | SingleNodeSingleArrowCatArrow
singleNodeSingleArrowCatEval :: a -> SingleNodeSingleArrowCatArrow a b -> SingleNodeCatObj b
singleNodeSingleArrowCatEval _ _ = fmap Prelude.id SingleNodeCatObj


instance Category SingleNodeSingleArrowCatArrow where
    id = SingleNodeSingleArrowCatId
    (.) (SingleNodeSingleArrowCatArrow) (SingleNodeSingleArrowCatId) = SingleNodeSingleArrowCatArrow
    (.) (SingleNodeSingleArrowCatId) (SingleNodeSingleArrowCatArrow) = SingleNodeSingleArrowCatArrow
    (.) (SingleNodeSingleArrowCatId) (SingleNodeSingleArrowCatId) = SingleNodeSingleArrowCatId


data TwoNodeCatObj = TwoNode0 | TwoNode1

data TwoNodeCatArrow a b = TwoNodeCatArrow01 | TwoNodeCatId
twoNodeCatEval :: TwoNodeCatObj -> TwoNodeCatArrow a b -> TwoNodeCatObj
twoNodeCatEval (TwoNode0) (TwoNodeCatArrow01) = TwoNode1
twoNodeCatEval (TwoNode0) (TwoNodeCatId) = TwoNode0
twoNodeCatEval (TwoNode1) (TwoNodeCatId) = TwoNode1

instance Category TwoNodeCatArrow where
    id = TwoNodeCatId
    (.) (TwoNodeCatArrow01) (TwoNodeCatId) = TwoNodeCatArrow01
    (.) (TwoNodeCatId) (TwoNodeCatArrow01) = TwoNodeCatArrow01
    (.) (TwoNodeCatId) (TwoNodeCatId) = TwoNodeCatId