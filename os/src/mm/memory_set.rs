use alloc::vec::Vec;

use super::page_table::PageTable;

pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
}

impl MemorySet {
    pub fn new() -> Self {
        MemorySet {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    // pub fn new_kernel() -> Self {
    //     extern "C" {
    //         fn __text_start();
    //         fn __text_end();
    //         fn __rodata_start();
    //         fn __rodata_end();
    //         fn __data_start();
    //         fn __data_end();
    //         fn __bss_start();
    //         fn __bss_end();
    //     }

    //     let mut memory_set = Self::new();
    //     // map kernel sections
    //     println!(".text [{:#x}, {:#x})", __text_start as usize, __text_end as usize);
    //     println!("mapping .text section");
    //     memory_set.push(
    //         MapArea::new(
    //             (stext as usize).into(),
    //             (etext as usize).into(),
    //             MapType::Identical,
    //             MapPermission::R | MapPermission::X,
    //         ),
    //         None,
    //     );

    //     println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    //     println!("mapping .rodata section");
    //     memory_set.push(
    //         MapArea::new(
    //             (srodata as usize).into(),
    //             (erodata as usize).into(),
    //             MapType::Identical,
    //             MapPermission::R,
    //         ),
    //         None,
    //     );

    //     println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    //     println!("mapping .data section");
    //     memory_set.push(
    //         MapArea::new(
    //             (sdata as usize).into(),
    //             (edata as usize).into(),
    //             MapType::Identical,
    //             MapPermission::R | MapPermission::W,
    //         ),
    //         None,
    //     );

    //     println!(
    //         ".bss [{:#x}, {:#x})",
    //         sbss_with_stack as usize, ebss as usize
    //     );
    //     println!("mapping .bss section");
    //     memory_set.push(
    //         MapArea::new(
    //             (sbss_with_stack as usize).into(),
    //             (ebss as usize).into(),
    //             MapType::Identical,
    //             MapPermission::R | MapPermission::W,
    //         ),
    //         None,
    //     );

    //     println!("mapping physical memory");
    //     memory_set.push(
    //         MapArea::new(
    //             (ekernel as usize).into(),
    //             MEMORY_END.into(),
    //             MapType::Identical,
    //             MapPermission::R | MapPermission::W,
    //         ),
    //         None,
    //     );
    //     memory_set
    // }

    // fn push(&mut self, map_area: MapArea, data: Option<&[u8]>) {
    //     map_area.map();
    //     if let Some(data) = data {
    //         map_area.copy_data(data);
    //     }
    //     self.areas.push(map_area);
    // }

    pub fn activate(&self) {
        let root_ppn = self.page_table.root_ppn;
        let satp = (8 << 60) | (root_ppn.value >> 12);
        unsafe {
            core::arch::asm!("csrw satp, {}", in(reg) satp);
            core::arch::asm!("sfence.vma");
        }
    }
}

struct MapArea {}
