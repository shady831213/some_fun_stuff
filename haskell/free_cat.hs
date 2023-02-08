-- category-theory-for-programmers challenge 3.6
module FreeCat where
import Control.Category

data SingleNodeCat = SingleNodeCat

data SingleNodeCatArrow a b = SingleNodeCatArrow(SingleNodeCat, SingleNodeCat, Integer)

instance Category SingleNodeCatArrow where
    id = SingleNodeCatArrow(SingleNodeCat, SingleNodeCat, 0)
    (.) _ _ = undefined



data SingleNodeSingleArrowCatArrow a b = SingleNodeSingleArrowCatArrow(SingleNodeCat, SingleNodeCat, Integer)
singleNodeSingleArrowCatArrowStart :: SingleNodeSingleArrowCatArrow a b -> SingleNodeCat
singleNodeSingleArrowCatArrowStart (SingleNodeSingleArrowCatArrow(s, _, _)) = s
singleNodeSingleArrowCatArrowEnd::SingleNodeSingleArrowCatArrow a b -> SingleNodeCat
singleNodeSingleArrowCatArrowEnd (SingleNodeSingleArrowCatArrow(_, e, _)) = e
singleNodeSingleArrowCatArrowId::SingleNodeSingleArrowCatArrow a b -> Integer
singleNodeSingleArrowCatArrowId (SingleNodeSingleArrowCatArrow(_, _, i)) = i

singleNodeSingleArrowCatId :: SingleNodeSingleArrowCatArrow a b
singleNodeSingleArrowCatId = SingleNodeSingleArrowCatArrow(SingleNodeCat, SingleNodeCat, 0)

singleNodeSingleArrowCatArrow1 :: SingleNodeSingleArrowCatArrow a b
singleNodeSingleArrowCatArrow1 = SingleNodeSingleArrowCatArrow(SingleNodeCat, SingleNodeCat, 1)

instance Category SingleNodeSingleArrowCatArrow where
    id = singleNodeSingleArrowCatId
    (.) a b = let   ida = singleNodeSingleArrowCatArrowId a
                    idb = singleNodeSingleArrowCatArrowId b 
                    in let  id = if ida > idb 
                                    then ida 
                                    else idb 
                    in SingleNodeSingleArrowCatArrow(singleNodeSingleArrowCatArrowStart a, singleNodeSingleArrowCatArrowEnd b, id)
