-- category-theory-for-programmers challenge 3.6
module FreeCat where
import Control.Category
newtype Node = Node String
data SingleNodeCat a b = SingleNodeCat Node
singleNodeCatObj :: SingleNodeCat a b
singleNodeCatObj = SingleNodeCat(Node "")
instance Category SingleNodeCat where
    id = singleNodeCatObj
    (.) _ _ = singleNodeCatObj