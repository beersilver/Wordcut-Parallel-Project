##Wordcut Algorithm in Parallel with Rayon

I forked the repo from veer66/chamkho, and tried to implement the code to be in parallel with rayon libray.

This program can amazingly separate Thai single words from a long adjacent sentence!

#Example
ผู้นำโง่เราจะตายกันหมด => ผู้|นำ|โง่|เรา|จะ|ตาย|กัน|หมด

กล้ามาก เก่งมาก ขอบใจ => กล้า|มาก| |เก่ง|มาก| |ขอบ|ใจ

You can see the changed code in the file named 'cli.rs' from line 53.




---- Original README below -----

# Chamkho
Lao/Thai word segmentation library in Rust

# Usage

## As command line

    echo "กากกา" | wordcut 

### External dictionary

    echo "กากกา" | wordcut -d <path to dictionary>

### Specific language

    echo "ພາສາລາວມີ" | wordcut -l lao
