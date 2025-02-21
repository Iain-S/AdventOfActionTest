module Main where

import Data.List (sort)
import qualified Data.List.NonEmpty as NE
import Data.Maybe (fromMaybe)
import Data.Text (Text)
import qualified Data.Text as T
import qualified Data.Text.IO as TIO
import Data.Text.Read (decimal)
import System.Environment (getArgs)

readLines :: IO [Text]
readLines = T.lines <$> TIO.readFile "input.txt"

parseLine :: Text -> (Int, Int)
parseLine t =
  case T.words t of
    [at, bt] -> case (decimal at, decimal bt) of
      (Right (ai, _), Right (bi, _)) -> (ai, bi)
      _ -> error "parse error"
    _ -> error "parse error"

parseInput :: IO ([Int], [Int])
parseInput = do
  ts <- fmap parseLine <$> readLines
  pure (sort $ map fst ts, sort $ map snd ts)

part1 :: IO ()
part1 = do
  (fsts, snds) <- parseInput
  print $ sum $ map abs $ zipWith (-) fsts snds

groupWithCount :: [Int] -> [(Int, Int)]
groupWithCount is =
  let groups = NE.group is
   in map (\g -> (NE.head g, length g)) groups

getCount :: Int -> [(Int, Int)] -> Int
getCount n gs = fromMaybe 0 (lookup n gs)

part2 :: IO ()
part2 = do
  (fsts, snds) <- parseInput
  let fstGroups = groupWithCount fsts
      sndGroups = groupWithCount snds
  print $ sum $ map (\(n, counts) -> n * counts * getCount n sndGroups) fstGroups

main :: IO ()
main = do
  args <- getArgs
  case args of
    [] -> part1 >> part2
    ["one"] -> part1
    ["two"] -> part2
    _ -> error "invalid arguments"
