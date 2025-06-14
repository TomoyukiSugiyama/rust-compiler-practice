fn main(){
    a=0;
    let sumi=0;
    let sumj=0;
    for ( i=0; i<5; i=i+1 ) {
        sumi=sumi+i;
        for ( j=0; j<5; j=j+1 ) {
            sumj=sumj+j;
        }
    }
    return sumi+sumj;
} 