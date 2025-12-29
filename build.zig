const std = @import("std");

pub fn build(b: *std.Build) void {

    // RISC-V 64-bit, no OS
    const target = b.standardTargetOptions(.{
        .default_target = .{
            .cpu_arch = .riscv64,
            .os_tag = .freestanding,
        },
    });

    const optimize = .Debug;

    const dtb = b.dependency("dtb", .{});

    const kernel = b.addExecutable(.{
        .name = "kernel",
        .root_module = b.createModule(.{
            .root_source_file = b.path("kernel/kmain.zig"),
            .code_model = .medium,
            .target = target,
            .optimize = optimize,
            .strip = false,
        }),
    });

    kernel.addAssemblyFile(b.path("kernel/start.s"));
    kernel.setLinkerScript(b.path("kernel/linker.ld"));

    kernel.root_module.addImport("dtb", dtb.module("dtb"));

    b.installArtifact(kernel);

    const run_kernel = b.addSystemCommand(&[_][]const u8 {
        "qemu-system-riscv64",
        "-machine", "virt",
        "-bios", "default",
        "-nographic",
        "-serial", "mon:stdio",
        "--no-reboot",
    });

    run_kernel.addArg("-kernel");
    run_kernel.addFileArg(kernel.getEmittedBin());
    run_kernel.step.dependOn(&kernel.step);

    const run_step = b.step("run", "Run the kernel in qemu");
    run_step.dependOn(&run_kernel.step);
}
