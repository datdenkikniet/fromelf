use object::{read::elf::ElfFile32, LittleEndian, Object, ObjectSection};
use std::path::Path;

pub enum Error {
    Object(object::Error),
    SectionMissing(&'static str),
    SectionDataMissing(&'static str),
    Io(std::io::Error),
}

pub fn elf_bytes_to_bin(elf_bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let object = ElfFile32::<LittleEndian>::parse(elf_bytes).map_err(|e| Error::Object(e))?;

    const SECTIONS: [&'static str; 4] = [".vector_table", ".text", ".rodata", ".data"];

    let mut bin = Vec::new();

    for section_name in SECTIONS {
        let log_name = section_name;
        log::debug!("{log_name}: searching object file for section...");

        let section = object
            .section_by_name(section_name)
            .ok_or(Error::SectionMissing(section_name))?;

        let section_data = section
            .uncompressed_data()
            .map_err(|_| Error::SectionDataMissing(section_name))?;

        log::info!("{log_name}: Adding data to binary blob.");

        let padding = if bin.len() as u64 % section.align() != 0 {
            section.align() - (bin.len() as u64 % section.align())
        } else {
            0
        };

        if padding > 0 {
            log::debug!(
                "{log_name}: Added {padding} bytes of padding to meet alignment of {}.",
                section.align(),
            );
        } else {
            log::debug!(
                "{log_name}: not padding as alignment of {} is met.",
                section.align()
            )
        }

        (0..padding).for_each(|_| bin.push(0));
        bin.extend_from_slice(&section_data);
    }

    Ok(bin)
}

pub fn elf_file_to_bin<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    let data = std::fs::read(path).map_err(|e| Error::Io(e))?;

    elf_bytes_to_bin(&data)
}
