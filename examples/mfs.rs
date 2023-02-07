use logger::init_logger;
use rvfs::ramfs::tmpfs::tmp_fs_type;
use rvfs::{
    do_mount, init_vfs, lookup_filesystem, path_walk, register_filesystem, vfs_mkdir,
    vfs_open_file, vfs_read_file, vfs_write_file, FakeFSC, FileFlags, FileMode, LookUpFlags,
    MountFlags,
};

fn main() {
    init_logger();

    println!("init vfs");
    init_vfs();
    println!("init vfs ok ......");
    // let lookup_data = path_walk::<FakeFSC>("/", LookUpFlags::DIRECTORY).unwrap();
    // println!("lookup_data: {:#?}", lookup_data);

    println!("--------------------------------------");
    println!("mkdir /tmp");
    vfs_mkdir::<FakeFSC>("/tmp", FileMode::FMODE_WRITE).unwrap();
    println!("mkdir /tmp ok ......");

    println!("--------------------------------------");
    println!("test path_walk /tmp");
    let _temp_find = path_walk::<FakeFSC>("/tmp", LookUpFlags::DIRECTORY).unwrap();
    println!("test path_walk /tmp ok ......");
    // println!("temp_find: {:#?}",temp_find);

    println!("--------------------------------------");
    // open exist file
    // let file = open_file::<FakeFSC>("/tmp", FileFlags::O_RDWR,FileMode::FMODE_READ).unwrap();
    // println!("file: {:#?}",file);
    println!("test create file /f1");
    // open or create file
    let file = vfs_open_file::<FakeFSC>(
        "/f1",
        FileFlags::O_RDWR | FileFlags::O_CREAT,
        FileMode::FMODE_WRITE | FileMode::FMODE_READ,
    )
    .unwrap();
    println!("test create file /f1 ok ......");

    println!("--------------------------------------");
    println!("test read/write file");
    // test read / write
    let mut buf = [0u8; 10];
    vfs_write_file::<FakeFSC>(file.clone(), [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].as_ref(), 0).unwrap();
    let _read = vfs_read_file::<FakeFSC>(file, buf.as_mut(), 0).unwrap();
    println!("read: {:?}", buf);
    println!("test read/write file ok ......");

    // 注册tmpfs，实际上也是一个内存文件系统，但这里的实现将其与rootfs分开了
    println!("----------------------------------------");
    register_filesystem(tmp_fs_type()).unwrap();
    println!("register tmpfs ok ......");
    println!("test do_mount");
    let tmpfs = do_mount::<FakeFSC>("", "/tmp", "tmpfs", MountFlags::MNT_NO_DEV, None).unwrap();
    // println!("mnt: {:#?}", mnt);
    println!("test do_mount ok ......");

    println!("----------------------------------------");
    println!("{:#?}", tmpfs);

    println!("----------------------------------------");
    println!("mkdir /tmp/tt1, it should in tmpfs root dir");

    vfs_mkdir::<FakeFSC>("/tmp/tt1", FileMode::FMODE_WRITE).unwrap();
    println!("mkdir /tmp/tt1 ok ......");

    println!("{:#?}", tmpfs);
}
