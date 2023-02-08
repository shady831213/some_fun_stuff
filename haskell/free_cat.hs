-- category-theory-for-programmers challenge 3.6.1
module FreeCat where
import Control.Category
import Data.Char

data SingleNodeCatObj = SingleNodeCatObj

data SingleNodeCatArrow a b = SingleNodeCatId
singleNodeCatEval :: SingleNodeCatObj -> SingleNodeCatArrow a b -> SingleNodeCatObj
singleNodeCatEval _ _ = SingleNodeCatObj

instance Category SingleNodeCatArrow where
    id = SingleNodeCatId
    (.) _ _ = undefined



data SingleNodeSingleArrowCatArrow a b = SingleNodeSingleArrowCatId | SingleNodeSingleArrowCatArrow
singleNodeSingleArrowCatEval :: SingleNodeCatObj -> SingleNodeSingleArrowCatArrow a b -> SingleNodeCatObj
singleNodeSingleArrowCatEval _ _ = SingleNodeCatObj


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

data SingleNodeMultiArrowCatArrow a b = SingleNodeMultiArrowCatId | SingleNodeMultiArrowCatArrow Char
singleNodeMultiArrowCatArrowMark :: SingleNodeMultiArrowCatArrow a b -> Maybe Char
singleNodeMultiArrowCatArrowMark (SingleNodeMultiArrowCatId) = Nothing
singleNodeMultiArrowCatArrowMark (SingleNodeMultiArrowCatArrow m) = Just m

singleNodeMultiArrowCatEval :: SingleNodeCatObj -> SingleNodeCatArrow a b -> SingleNodeCatObj
singleNodeMultiArrowCatEval _ _ = SingleNodeCatObj

instance Category SingleNodeMultiArrowCatArrow where
    id = SingleNodeMultiArrowCatId
    (.) (SingleNodeMultiArrowCatId) (SingleNodeMultiArrowCatArrow m) = SingleNodeMultiArrowCatArrow m
    (.) (SingleNodeMultiArrowCatArrow m) (SingleNodeMultiArrowCatId) = SingleNodeMultiArrowCatArrow m
    (.) (SingleNodeMultiArrowCatArrow m) (SingleNodeMultiArrowCatArrow n) = SingleNodeMultiArrowCatArrow (chr ((ord m + ord n) `mod` ord 'a'))