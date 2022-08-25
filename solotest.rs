fn main()
{
    let dizi[7][7]={{2,2,1,1,1,2,2},{2,2,1,1,1,2,2},{1,1,1,1,1,1,1},{1,1,1,0,1,1,1},{1,1,1,1,1,1,1},{2,2,1,1,1,2,2},{2,2,1,1,1,2,2}};

    while(1)
    {
        tahta(dizi);
        hareket(dizi);
        system("cls");
    }

    return 0;
}

fn tahta(int dizi[][7])
{
    let mut  i,j;
    let mut  eksen=1;

        println!("\t    ");

        for(i=0;i<7;i++)
            println!("%d   ",eksen++);

        println!("\n\n");

        for(i=0;i<7;i++)
        {
            println!("\t");
            println!("%d | ",i+1);
            for(j=0;j<7;j++)
            {
                if(dizi[i][j]==2)
                {
                println!("    ");
                }
                else
                {
                println!("%d   ",dizi[i][j]);
                }

            }
            println!("\n\n");
        }

    }

fn hareket(let mut dizi[][7])
{
    let mut  y1,y2;
    let mut  x1,x2;

        println!("X1 Degerini Giriniz: ");
        scan!("%d",&x1);
        println!("Y1 Degerini Giriniz: ");
        scan!("%d",&y1);
        println!("Tasi Atamak Istediginiz X2 Degerini Giriniz: ");
        scan!("%d",&x2);
        println!("Tasi Atamak Istediginiz Y2 Degerini Giriniz: ");
        scan!("%d",&y2);

        if(dizi[y1-1][x1-1]==1 && dizi[y1-1][x1]==1 && dizi[y2-1][x2-1]==0 && x2-x1==2)
            {
                dizi[y1-1][x1-1]=0;
                dizi[y1-1][x1]=0;
                dizi[y2-1][x2-1]=1;
            }
        else if(dizi[y1-1][x1-1]==1 && dizi[y1-1][x1-2]==1 && dizi[y2-1][x2-1]==0 && x1-x2==2)
            {
                dizi[y1-1][x1-1]=0;
                dizi[y1-1][x1-2]=0;
                dizi[y2-1][x2-1]=1;
            }
        else if(dizi[y1-1][x1-1]==1 && dizi[y1][x1-1]==1 && dizi[y2-1][x2-1]==0 && y2-y1==2)
            {
                dizi[y1-1][x1-1]=0;
                dizi[y1][x1-1]=0;
                dizi[y2-1][x2-1]=1;

            }
        else if(dizi[y1-1][x1-1]==1 && dizi[y1-2][x1-1]==1 && dizi[y2-1][x2-1]==0 && y1-y2==2)
            {
                dizi[y1-1][x1-1]=0;
                dizi[y1-2][x1-1]=0;
                dizi[y2-1][x2-1]=1;

            }
}
