inputDir="$(pwd)";
content="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lobortis elementum nibh tellus molestie nunc non blandit massa. Ut morbi tincidunt augue interdum velit euismod in. Sollicitudin aliquam ultrices sagittis orci a scelerisque purus semper eget. In tellus integer feugiat scelerisque varius morbi enim nunc faucibus. Eget lorem dolor sed viverra. Morbi tincidunt ornare massa eget egestas. Lacus suspendisse faucibus interdum posuere lorem ipsum. Libero enim sed faucibus turpis in eu mi. Augue interdum velit euismod in pellentesque massa. Euismod lacinia at quis risus sed vulputate. Ultrices sagittis orci a scelerisque purus semper eget. Nec feugiat nisl pretium fusce id velit ut tortor pretium. Auctor urna nunc id cursus metus. Elementum curabitur vitae nunc sed velit dignissim. Integer quis auctor elit sed vulputate mi sit amet mauris.\nPretium nibh ipsum consequat nisl vel pretium lectus quam. Dignissim sodales ut eu sem. Mauris nunc congue nisi vitae suscipit tellus mauris a diam. Ut lectus arcu bibendum at varius vel pharetra. Etiam non quam lacus suspendisse. Risus nullam eget felis eget nunc lobortis mattis. Augue neque gravida in fermentum. Egestas tellus rutrum tellus pellentesque eu tincidunt tortor aliquam nulla. Auctor elit sed vulputate mi sit. Ipsum faucibus vitae aliquet nec ullamcorper. Egestas sed sed risus pretium quam vulputate dignissim. Tempor orci eu lobortis elementum nibh. Malesuada fames ac turpis egestas maecenas pharetra. Nisl nisi scelerisque eu ultrices vitae auctor.\nSodales neque sodales ut etiam. Vel fringilla est ullamcorper eget nulla facilisi. Eu scelerisque felis imperdiet proin fermentum leo vel orci. Lorem donec massa sapien faucibus et molestie ac feugiat. Felis bibendum ut tristique et egestas quis ipsum. Nisi vitae suscipit tellus mauris. Nunc lobortis mattis aliquam faucibus purus in massa tempor. Et netus et malesuada fames ac turpis egestas sed. Aliquet lectus proin nibh nisl condimentum id. Pellentesque nec nam aliquam sem et tortor consequat id. Vel facilisis volutpat est velit egestas dui id ornare. Ultrices mi tempus imperdiet nulla malesuada pellentesque. Est velit egestas dui id ornare arcu odio ut sem. Egestas purus viverra accumsan in nisl nisi scelerisque eu ultrices. Risus in hendrerit gravida rutrum quisque.\nAuctor neque vitae tempus quam pellentesque nec nam aliquam sem. Nisl purus in mollis nunc sed id. Convallis posuere morbi leo urna molestie at elementum eu facilisis. Elit ullamcorper dignissim cras tincidunt lobortis feugiat. Purus faucibus ornare suspendisse sed nisi lacus sed viverra. Orci phasellus egestas tellus rutrum tellus pellentesque eu tincidunt. Orci dapibus ultrices in iaculis nunc sed. Gravida neque convallis a cras semper. Et leo duis ut diam. Neque laoreet suspendisse interdum consectetur libero id faucibus nisl tincidunt. Aliquet porttitor lacus luctus accumsan. Potenti nullam ac tortor vitae purus faucibus ornare suspendisse. Id semper risus in hendrerit gravida rutrum quisque.\nVulputate sapien nec sagittis aliquam malesuada bibendum arcu vitae elementum. Purus sit amet luctus venenatis lectus magna fringilla. Eros in cursus turpis massa tincidunt dui ut ornare lectus. Consectetur adipiscing elit duis tristique sollicitudin. Amet cursus sit amet dictum sit. Arcu bibendum at varius vel pharetra vel. Purus in massa tempor nec feugiat nisl pretium fusce. Enim tortor at auctor urna nunc id cursus metus aliquam. Ut etiam sit amet nisl purus in mollis nunc. Platea dictumst quisque sagittis purus sit amet volutpat consequat. Cras adipiscing enim eu turpis. Volutpat ac tincidunt vitae semper quis. At tellus at urna condimentum mattis pellentesque. Sit amet consectetur adipiscing elit duis tristique sollicitudin nibh sit. Adipiscing elit pellentesque habitant morbi."

populateFolder() {
  touch "file_from_function.txt"
  for i in {1..100}; do
    echo $content >> "file_from_function.txt"
  done

  for i in {1..30}; do
    cp "./file_from_function.txt" "./file_from_function_$i.txt"
  done
} 

mkdir testingDir;
cd testingDir;

for i in {1..20}; do
    mkdir "testingDir_$i";
    touch "testingFile_$i.txt";
    echo $content >> "testingFile_$i.txt";

    cd "testingDir_$i";
    populateFolder;

    cd "..";
done
