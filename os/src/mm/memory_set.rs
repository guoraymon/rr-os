use alloc::vec::Vec;

use crate::{
    mm::{KERNEL_SPACE, MEMORY_END},
    println,
};

use super::{
    address::{PhysPageNum, VirtAddr, VirtPageNum},
    page_table::{PageTable, PageTableEntryFlags},
};

extern "C" {
    fn __text_start();
    fn __text_end();
    fn __rodata_start();
    fn __rodata_end();
    fn __data_start();
    fn __data_end();
    fn __bss_start();
    fn __bss_end();
    fn __kernel_end();
}

pub struct MemorySet {
    pub page_table: PageTable,
    areas: Vec<MapArea>,
}

impl MemorySet {
    pub fn new() -> Self {
        MemorySet {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    pub fn new_kernel() -> Self {
        let mut memory_set = Self::new();
        // map kernel sections
        println!(
            ".text [{:#x}, {:#x})",
            __text_start as usize, __text_end as usize
        );
        println!("mapping .text section");
        memory_set.push(
            MapArea::new(
                VirtAddr::from(__text_start as usize),
                VirtAddr::from(__text_end as usize),
                MapPermission::R | MapPermission::X,
            ),
            None,
        );

        println!(
            ".rodata [{:#x}, {:#x})",
            __rodata_start as usize, __rodata_end as usize
        );
        println!("mapping .rodata section");
        memory_set.push(
            MapArea::new(
                VirtAddr::from(__rodata_start as usize),
                VirtAddr::from(__rodata_end as usize),
                MapPermission::R,
            ),
            None,
        );

        println!(
            ".data [{:#x}, {:#x})",
            __data_start as usize, __data_end as usize
        );
        println!("mapping .data section");
        memory_set.push(
            MapArea::new(
                VirtAddr::from(__data_start as usize),
                VirtAddr::from(__data_end as usize),
                MapPermission::R | MapPermission::W,
            ),
            None,
        );

        println!(
            ".bss [{:#x}, {:#x})",
            __bss_start as usize, __bss_end as usize
        );
        println!("mapping .bss section");
        memory_set.push(
            MapArea::new(
                VirtAddr::from(__bss_start as usize),
                VirtAddr::from(__bss_end as usize),
                MapPermission::R | MapPermission::W,
            ),
            None,
        );

        println!(
            "physical [{:#x}, {:#x})",
            __kernel_end as usize, MEMORY_END as usize
        );
        println!("mapping physical memory");
        memory_set.push(
            MapArea::new(
                VirtAddr::from(__kernel_end as usize),
                VirtAddr::from(MEMORY_END),
                MapPermission::R | MapPermission::W,
            ),
            None,
        );

        memory_set
    }

    fn push(&mut self, map_area: MapArea, data: Option<&[u8]>) {
        map_area.map(&mut self.page_table);
        if let Some(data) = data {
            todo!("copy data to map area");
        }
        self.areas.push(map_area);
    }

    pub fn activate(&self) {
        let root_ppn = self.page_table.root_ppn;
        let satp = (8 << 60) | root_ppn.0;
        unsafe {
            core::arch::asm!(
                "csrw satp, {satp}",
                "sfence.vma",
                satp = in(reg) satp,
            );
        }
        println!("satp activated!")
    }
}

struct MapArea {
    vpn_range: (VirtPageNum, VirtPageNum),
    map_perm: MapPermission,
}

impl MapArea {
    pub fn new(start_va: VirtAddr, end_va: VirtAddr, map_perm: MapPermission) -> Self {
        let start_va = start_va.floor();
        let end_va = end_va.ceil();
        Self {
            vpn_range: (start_va.clone().into(), end_va.clone().into()),
            map_perm,
        }
    }

    fn map(&self, page_table: &mut PageTable) {
        let start_vpn = self.vpn_range.0;
        let end_vpn = self.vpn_range.1;
        for vpn in start_vpn.0..end_vpn.0 {
            // println!("mapping vpn {:#x}", vpn);
            page_table.map(
                VirtPageNum::from(vpn),
                PhysPageNum::from(vpn),
                self.map_perm.to_pte_flags(),
            );
        }
    }

    fn unmap(&self, page_table: &mut PageTable) {
        for vpn in (self.vpn_range.0).0..(self.vpn_range.1).0 {
            page_table.unmap(VirtPageNum::from(vpn));
        }
    }
}

pub enum MapType {
    Identical,
    Framed,
}

#[derive(Clone, Copy)]
pub struct MapPermission(u8);

impl MapPermission {
    pub const R: Self = Self(1 << 1);
    pub const W: Self = Self(1 << 2);
    pub const X: Self = Self(1 << 3);
    pub const U: Self = Self(1 << 4);

    pub fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    pub fn to_pte_flags(self) -> PageTableEntryFlags {
        let mut flags = PageTableEntryFlags::V;

        if self.contains(MapPermission::R) {
            flags = flags | PageTableEntryFlags::R;
        }
        if self.contains(MapPermission::W) {
            flags = flags | PageTableEntryFlags::W;
        }
        if self.contains(MapPermission::X) {
            flags = flags | PageTableEntryFlags::X;
        }
        if self.contains(MapPermission::U) {
            flags = flags | PageTableEntryFlags::U;
        }

        flags
    }
}

impl core::ops::BitOr for MapPermission {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

pub fn remap_test() {
    let mid_text: VirtAddr = ((__text_start as usize + __text_start as usize) / 2).into();
    assert!(!KERNEL_SPACE
        .page_table
        .translate(mid_text.floor())
        .is_writeable(),);

    let mid_rodata: VirtAddr = ((__rodata_end as usize + __rodata_end as usize) / 2).into();
    assert!(!KERNEL_SPACE
        .page_table
        .translate(mid_rodata.floor())
        .is_writeable(),);

    let mid_data: VirtAddr = ((__data_start as usize + __data_end as usize) / 2).into();
    assert!(!KERNEL_SPACE
        .page_table
        .translate(mid_data.floor())
        .is_executable(),);

    println!("remap_test passed!");
}
