Belakangan ini saya kepikiran nyari solusi untuk dua hal yang remeh, tapi lumayan sering kepikiran kalau lagi pakai Mac, yaitu males buka activity monitor cuma buat ngecek suhu, dan sering nyesek sendiri kalau teks panjang yang baru aja di-copy malah hilang gara-gara ketimpa copy-an yang baru.

Biasa sih solusinya tinggal cari aplikasinya di internet. Tapi dipikir-pikir, mumpung zaman sekarang udah ada AI, kenapa nggak coba bikin sendiri aja?

Sekalian juga jadi tantangan buat nyobain ngoding pakai Rust. Niat saya simpel: saya mau aplikasi utilities yang benar-benar enteng, nggak rakus resource, dan cuma jalan sunyi di menu bar aja.

Kerjaan sehari-hari saya biasa ngoprek Android (Kotlin), jadi kalau harus bikin Mac desktop app yang native dari nol pasti repot. Tapi di sinilah AI kerasa banget gunanya. Saya jujur nggak sampai *deep dive* belajar low-level API-nya Apple. Saya cukup pegang logic *high-level*-nya aja, terus biarin AI yang ngerjain kodingan kasarnya. Bikin aplikasi native lintas platform yang tadinya ribet jadi bisa cepet selesai tanpa perlu pusing ngulik dokumentasi.

Dari sekadar ngide nemuin solusi buat diri sendiri, lumayan seneng akhirnya malah tembus ngerilis 2 native app kecil-kecilan:

Yang pertama ada **mactemp** (https://github.com/firdaus1453/mactemp). Ini aplikasi di menu bar buat mantau suhu CPU/GPU sama RAM. Asli ini ringan banget, ukuran file-nya cuma sekitar **366 KB**.

Terus yang kedua **CopyCan** (https://github.com/firdaus1453/CopyCan). Yang ini clipboard history manager. Kerjanya murni ngumpet, begitu kita pencet shortcut Cmd+Shift+V, list copy-an kita langsung muncul tepat di titik kursor mouse berada. Ukurannya? Cuma sekitar **880 KB**.

Bisa ngebangun desktop GUI app di ekosistem Mac tapi size file-nya masih di rentang Kilobyte itu punya kepuasan batin sendiri sih 😂. Belakangan saya baru ngeh, ternyata di luar sana udah banyak app terkenal buat urusan ginian (misal: *Maccy* buat clipboard, atau *Stats* & *Hot* buat monitor system). Mereka app yang keren, tapi karena pakai framework besar, ukurannya bisa belasan sampai puluhan Megabyte. Sama sekali nggak nyangka kalau ngide ngebangun pakai Rust bakal nekan sizenya se-ekstrim ini sampai tembus di bawah 1 MB!

Biar sekalian lengkap, dua-duanya juga udah saya pasangin CI/CD pakai GitHub actions, jadi tiap rilis otomatis ngebundle langsung siap didownload bentuk `.dmg`.

AI sukses ngebuka pintu buat ngerasain tech stack yang tadinya kerasa kejauhan. Lewat bantuan ini, ide-ide sederhana jadi jauh lebih gampang buat dieksekusi jadi barang nyata.

Buat temen-temen yang kebetulan butuh tools ringan kayak gini, silakan mampir ke repo saya yak. Keduanya 100% open source!

#rust #macos #softwareengineering #sideproject #ai
