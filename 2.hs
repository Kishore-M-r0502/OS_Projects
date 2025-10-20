data Point = Point Double Double
    deriving (Show , Eq)
class Coordinate a where
    right :: a -> a
    down :: a -> a
    up :: a -> a
    left :: a -> a

instance Coordinate Point where
    right (Point x y) = Point (x+1) y
    down (Point x y) = Point x (y-1)
    up (Point x y) = Point x (y+1)
    left (Point x y) = Point (x-1) y

distance ::  Point  -> Point  -> Double
distance (Point a b) (Point c d) = sqrt((c-a)^2+(d-b)^2)

