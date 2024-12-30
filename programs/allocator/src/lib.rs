use anchor_lang::prelude::*;
use std::alloc::{GlobalAlloc, Layout};

pub mod customize_allocator;

pub use crate::customize_allocator::CustomizeBumpAllocator;
declare_id!("6atnv2uSSvZ8byveP2xeYfwGUDwRDcXBqtRKrphGXRXR");

#[program]
pub mod allocator {

    use super::*;

    pub fn heap_allocate(_ctx: Context<HeapAllocateCtx>, size: u64) -> Result<u64> {
        msg!("Heap Allocate: Allocating {} bytes", size);
        let data = vec![0u8; size as usize];
        msg!("Heap Allocation Completed. Size: {} bytes", data.len());
        Ok(data.len() as u64)
    }

    pub fn customize_allocate(_ctx: Context<CustomizeAllocateCtx>, size: u64) -> Result<u64> {
        msg!("Bump Allocate: Allocating {} bytes", size);
        unsafe {
            let layout = Layout::from_size_align(size as usize, 8).unwrap();
            let ptr = CustomizeBumpAllocator.alloc(layout);
            if ptr.is_null() {
                msg!("Bump Allocation Failed.");
                return Err(ErrorCode::AllocationFailed.into());
            }
            std::ptr::write_bytes(ptr, 0, size as usize);
        }
        msg!("Bump Allocation Completed. Size: {} bytes", size);
        Ok(size as u64)
    }
}

#[derive(Accounts)]
pub struct HeapAllocateCtx {}

#[derive(Accounts)]
pub struct CustomizeAllocateCtx {}

#[error_code]
pub enum ErrorCode {
    #[msg("Memory allocation failed.")]
    AllocationFailed,
}
