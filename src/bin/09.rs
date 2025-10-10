advent_of_code::solution!(9);

#[derive(Debug, Clone)]
struct Block {
    id: Option<u64>,
    size: usize,
    position: usize,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks = parse_disk(input.trim());

    compact_disk(&mut blocks);

    let checksum = blocks
        .iter()
        .enumerate()
        .filter_map(|(pos, &file_id)| file_id.map(|id| pos as u64 * id))
        .sum();

    Some(checksum)
}

fn parse_disk(disk: &str) -> Vec<Option<u64>> {
    let mut blocks = Vec::new();
    let mut file_id = 0;

    for (i, ch) in disk.chars().enumerate() {
        let length = ch.to_digit(10).unwrap() as usize;

        if i % 2 == 0 {
            for _ in 0..length {
                blocks.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..length {
                blocks.push(None);
            }
        }
    }

    blocks
}

fn compact_disk(blocks: &mut Vec<Option<u64>>) {
    let mut left: usize = 0;
    let mut right = blocks.len() - 1;
    while left < right {
        while blocks[left].is_some() {
            left += 1;
        }
        while blocks[right].is_none() {
            right -= 1;
        }
        if left < right {
            blocks[left] = blocks[right];
            blocks[right] = None;
            left += 1;
            right -= 1;
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut segments = parse_disk_segments(input.trim());
    compact_whole_files(&mut segments);
    let blocks = segments_to_blocks(&segments);
    let checksum = blocks
        .iter()
        .enumerate()
        .filter_map(|(pos, &file_id)| file_id.map(|id| pos as u64 * id))
        .sum();
    Some(checksum)
}

fn parse_disk_segments(disk: &str) -> Vec<Block> {
    let mut segments = Vec::new();
    let mut file_id = 0;
    let mut position = 0;

    for (i, ch) in disk.chars().enumerate() {
        let size = ch.to_digit(10).unwrap() as usize;

        if size > 0 {
            if i % 2 == 0 {
                // 文件段
                segments.push(Block {
                    id: Some(file_id),
                    size,
                    position,
                });
                file_id += 1;
            } else {
                // 空闲段
                segments.push(Block {
                    id: None,
                    size,
                    position,
                });
            }
        }
        position += size;
    }
    segments
}

fn compact_whole_files(segments: &mut Vec<Block>) {
    let mut file_ids: Vec<u64> = segments.iter().filter_map(|seg| seg.id).collect();
    file_ids.sort_by(|a, b| b.cmp(a)); //descending

    for file_id in file_ids {
        let file_index = segments
            .iter()
            .position(|seg| seg.id == Some(file_id))
            .unwrap();

        let file_size = segments[file_index].size;

        for i in 0..segments.len() {
            let old_free_size = segments[i].size;
            if segments[i].id.is_none() && segments[i].size >= file_size {
                segments[i] = Block {
                    id: Some(file_id),
                    size: file_size,
                    position: segments[i].position,
                };

                segments[file_index] = Block {
                    id: None,
                    size: file_size,
                    position: segments[file_index].position,
                };

                if old_free_size > file_size {
                    segments.insert(
                        i + 1,
                        Block {
                            id: None,
                            size: old_free_size - file_size,
                            position: segments[i].position + file_size,
                        },
                    );
                }

                break;
            }
        }
    }
}

fn segments_to_blocks(segments: &[Block]) -> Vec<Option<u64>> {
    let total_size = segments.iter().map(|seg| seg.size).sum();
    let mut blocks = vec![None; total_size];

    for segment in segments {
        for i in 0..segment.size {
            blocks[segment.position + i] = segment.id;
        }
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
