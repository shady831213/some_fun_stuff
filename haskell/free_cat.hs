-- category-theory-for-programmers challenge 3.6
module FreeCat where
import Control.Category

data SingleNodeCat = SingleNodeCat

data SingleNodeCatArrow a b = SingleNodeCatArrow(SingleNodeCat -> SingleNodeCat)
signleNodeId::SingleNodeCat -> SingleNodeCat
signleNodeId o = o
instance Category SingleNodeCatArrow where
    id = SingleNodeCatArrow(signleNodeId)
    (.) _ _ = undefined


-- data SingleNodeSingleArrow a b = SingleNodeSingleArrow(SingleNodeCat, SingleNodeCat)

-- instance Category SingleNodeSingleArrow where
--     id = SingleNodeCatId(SingleNodeCat, SingleNodeCat)
--     (.) _ _ = SingleNodeSingleArrow(SingleNodeCat, SingleNodeCat)