use colored::Color;

pub struct Distro {
    pub name: &'static str,
    pub description: &'static str,
    pub color: colored::Color,
    pub is_bold: bool
}

pub fn get_distros() -> Vec<Distro> {
    vec![/* 
        Distro {
            name: "Ubuntu",
            description: "Ubuntu is a popular Linux distribution based on Debian. It used to be really nice, but then they added snaps and a lot of bloat. Still a solid choice",
            color: Color::Red,
            is_bold: false,
        },
        Distro {
            name: "Fedora",
            description: "Fedora is a cutting-edge Linux distribution sponsored by Red Hat. Solid choice, even Linus Torvalds uses it!",
            color: Color::Blue,
            is_bold: true,
        },
        Distro {
            name: "Debian",
            description: "Debian is a stable and versatile Linux distribution. Pretty much the holy grail of modern distros.",
            color: Color::Red,
            is_bold: false,
        },
        Distro {
            name: "Arch Linux",
            description: "Arch Linux is a lightweight and flexible Linux distribution. Go ahead, say the words, you use Arch btw..",
            color: Color::Cyan,
            is_bold: true,
        },
        Distro {
            name: "openSUSE",
            description: "openSUSE is a versatile and stable Linux distribution. And I like the little chameleon",
            color: Color::BrightGreen,
            is_bold: true,
        },
        Distro {
            name: "Linux Mint",
            description: "Linux Mint is a user-friendly Linux distribution based on Ubuntu. Really a solid choice if you want to get work done. Used this one a long time myself! :D",
            color: Color::Green,
            is_bold: false,
        },
        Distro {
            name: "Gentoo",
            description: "Gentoo is a flexible and source-based Linux distribution. Have fun compling stuff for hours haha!",
            color: Color::Magenta,
            is_bold: true,
        },
        Distro {
            name: "Void Linux",
            description: "Void Linux is an independent Linux distribution focused on simplicity. Extra points because this one sounds really cool",
            color: Color::Green,
            is_bold: true,
        },
        Distro {
            name: "EndeavourOS",
            description: "EndeavourOS is Arch but without all the pain. Really nice choice! :)",
            color: Color::Magenta,
            is_bold: true,
        },
        Distro {
            name: "NixOS",
            description: "NixOS is a Linux distribution based on Nix package manager. It's really cool if you like to manage your system with code and stuff",
            color: Color::BrightMagenta,
            is_bold: true,
        },
        Distro {
            name: "Linux From Scratch",
            description: "Linux From Scratch huh? So you have chosen death. Just kidding, this one is really cool if you want to learn how Linux works from the ground up",
            color: Color::BrightRed,
            is_bold: false,
        },
        Distro {
            name: "OpenBSD",
            description: "OpenBSD is a Unix-like operating system focused on security, simplicity, and code correctness. It's a top choice for secure servers and firewalls.",
            color: Color::Yellow,
            is_bold: true,
        },
        Distro {
            name: "FreeBSD",
            description: "FreeBSD is a Unix-like operating system known for its performance and advanced networking capabilities. It's great for servers, desktops, and embedded systems.",
            color: Color::White,
            is_bold: false,
        },*/
        Distro {
            name: "Kali Linux",
            description: "The OS for Hackers and Script Kiddies. Based on Debian and equipped with a wide range of Hacking and pentesting Tools, it is the perfect choice for Hackers and everybody who wants to become one.",
            color: Color::BrightBlue,
            is_bold: true,
        },
        Distro {
            name: "Qubes OS",
            description: "A Secure OS running everything in virtual Machines to protect against hacking attacks and everything else.",
            color: Color::BrightWhite,
            is_bold: false,
        },
        Distro {
            name: "Tails OS",
            description: "Put on your tinfoil hat. Tails OS is an anonymous, live OS with built in integration of Tor and I2P for the maximum of privacy",
            color: Color::BrightBlack,
            is_bold: true,
        },
        Distro {
            name: "Vanilla OS",
            description: "Vanilla OS, allows you to install packages from most other Linux distros while staying immutable.",
            color: Color::BrightYellow,
            is_bold: false,
        },
    ]
}
