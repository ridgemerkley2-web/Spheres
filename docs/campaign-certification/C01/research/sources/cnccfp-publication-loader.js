// ***************************
//  DONNÉES JSON via Ajax
// ***************************

	
$(document).ready(function(){


	// 20221018 gestion des publications plus anciennes : 
	
	var annee_actuelle = new Date().getFullYear();
	var annee_last_publication = annee_actuelle - 2; // l'exercice est sur n-1;
	
	//console.log("Année actuelle : "+annee_actuelle)
	//console.log("Année last publication "+annee_last_publication);
	
	
	var options = "<option disabled>Choisir une année d'exercice</option>";
	
	var annee_choisie = "";
	
	for (i=annee_last_publication; i > 2017; i--) {
	
		var j = i;
		var annee = j.toString();
	
		if (window.location.href.indexOf(annee) != -1) { 
	
			annee_choisie = annee;
	
		}
	
	}
	
	// console.log('ANNÉE CHOISIE : '+annee_choisie);
	
	$("h1#titre_page").text("Comptes "+annee_choisie+" des partis politiques - publication");
	
	$("#notice_lien").prop('href', 'cnccfp_notice_publication_comptes_des_partis_'+ annee_choisie +'.pdf');
	
	for (i=annee_last_publication; i > 2017; i--) { // (premier exercice publié en ligne : 2018)
	
		// console.log(i);
		var j = i;
		var annee = j.toString();
		var option_choisie = "";
		
		
		// // console.log(annee);
		
		if (annee == annee_choisie) {
		
			option_choisie = " selected";
			// // console.log('année = année choisie');
		
		}
		
		//console.log("option : "+annee+ " - choisie : "+option_choisie);
		
		
		
		options += '<option'+option_choisie+'>'+annee+'</option>';
		
		if (i == annee_last_publication) { // vérification de l'existence des données json
			
			fichier_json = 'comptes_partis_'+annee+'.txt';
			
			$.ajax({
        		url:'https://liste.cnccfp.fr/publications/'+fichier_json,
				error: function()
				{
				   // // console.log("Le fichier n'existe pas encore - redirection vers année précédente :");
				   annee_max = annee_last_publication-1;
				   annee_max_txt = annee_max.toString();
				   
				   // // console.log(annee_max_txt);
					
					$('select#annee_exercice option').eq(1).remove(); // suppression de la première option proposant une année à choisir (la seconde donc)
				   
				},
				success: function()
				{
					
					// // console.log("Le fichier existe");
					// // console.log("vérification options dans ajax :"+options)
							
				}
			});
		
		}
		
	
	}
	
	options += "<option disabled>Avant 2018 se reporter au Journal officiel</option>";
	
	// // console.log('OPTIONS : '+options);
	
	$("select#annee_exercice").html(options);
	
	$('#annee_exercice').select2();
	
	// $('#annee_exercice').trigger('change.select2');

	$("#reset").on("click", function() {

		$('#rechercher').val('');
		$('#rechercher').trigger('click');

	});
	
	// ***************************
	//  OBJECT (JSON via Ajax)
	// ***************************
	
	
	$('#table_json').tablesorter({
		theme: 'blue',
		//sortStable : true,
		StringTo: "max",
		 sortLocaleCompare : true,
// maintain a stable sort (First Name column)
ignoreCase : true,
// if false, upper case sorts BEFORE lower case
		sortList: [[1,0]],
		//sortList: [[0,0], [1,0], [2,0], [3,0], [5,0], [6,0]],
		//sortForce: [[5,0]],
		widgets: ['zebra','stickyHeaders'],
		widgetOptions: {
			build_type   : 'json',
			build_source : { url: 'comptes_partis_'+annee_choisie+'.txt', dataType: 'json' },
			build_complete : 'tablesorter-build-complete',
			stickyHeaders : 'tablesorter-stickyHeader'
		}
	}); // BL MAJ annee_choisie

	
	$('table').bind('tablesorter-build-complete', function() {

			//alert($("td:nth-child(0)").html());
			$("table td:nth-child(1)").addClass("id");
			$("table td:nth-child(3)").addClass("cpte"); // MODIFIER index
			$("table td:nth-child(4)").addClass("v"); 
			$("table td:nth-child(5)").addClass("extrait");
			$("table td:nth-child(6)").addClass("obs");
			$("table td:nth-child(7)").addClass("dec");
			
			$("table th:nth-child(1)").prop("title","Numéro d’identification du parti politique attribué par la CNCCFP");
			$("table th:nth-child(2)").prop("title","Dénomination du parti politique à la date de l’exercice concerné");
			
			$("table th:nth-child(3)").prop("title","Dernière version complète des comptes du parti déposés à la CNCCFP (exercice "+annee_choisie+")"); // BL MAJ
			
			$("table th:nth-child(4)").prop("title","Numéro de la version des comptes déposés à la CNCCFP");
			
			$("table th:nth-child(5)").prop("title","Extrait des comptes annuels envoyé à la CNCCFP afin de compléter la dernière version des comptes déposés");
			
			$("table th:nth-child(6)").prop("title","Réserves et/ou observations du ou des commissaires aux comptes du parti ; observations de la CNCCFP");
			
			$("table th:nth-child(7)").prop("title","Décision de la commission portant sur le respect ou non par le parti de ses obligations légales.");

			$("table th:nth-child(8)").prop("title","Dépôt conforme (DC) ; comptes non déposés (AD) ; comptes déposés hors-délai (HD) ; comptes non certifiés (NC) ; comptes ne respectant pas le règlement comptable (ANC)");
		
			var classe_fichier = ['cpte','v','extrait','obs','dec'];
		
			for(var i= 0; i < classe_fichier.length; i++) {


				$("."+classe_fichier[i]).each(function() {
		
					var contenu = $(this).html();
					var img = '';
					var icone = '';
					var format_fichier = '';
			
					if (contenu != '' && classe_fichier[i] != 'v' && classe_fichier[i] != 'dec') {
					
						if (contenu.indexOf('.pdf') == -1 && contenu.indexOf('.PDF') == -1) {
							format_fichier = "Excel";
							//icone = '<i class="fa fa-file-excel-o fa-2x" aria-hidden="true" style="color:green;"></i>';
							icone = '<i class="far fa-file-excel fa-2x" aria-hidden="true" style="color:green;""></i>';
								
							// img = "xls_24";
								
						} else {
						
							format_fichier = "PDF";
							icone = '<i class="far fa-file-pdf fa-2x" aria-hidden="true" style="color:red;""></i>';
							
							// icone = '<i class="fa fa-file-pdf-o fa-2x" aria-hidden="true" style="color:red;"></i>';
							// img = "pdf_30";
						}
			
						if (i == 0) {
						
							if (contenu.indexOf('>Compte<') !== -1) { // 20240301
							
								nouveau = contenu.replace('>Compte<', ' title="Fichier '+format_fichier+'">'+icone+'<');
								
							} else {
								nouveau = contenu.replace('>Comptes<', ' title="Fichier '+format_fichier+'">'+icone+'<');
							}
						} else if (i == 2) {
							
								nouveau = contenu.replace('>Extrait<', ' title="Fichier '+format_fichier+'">'+icone+'<');
				
						} else if (i == 3) {
							nouveau = contenu.replace('>Observations<', ' title="Fichier '+format_fichier+'">'+icone+'<');
						}
						
			
						$(this).html(nouveau);
						
					
					} else if (contenu == 'non-respect') {
						
						$(this).addClass('non');
					
					} else if (contenu == '') {
			
						$(this).addClass('aucun');
					}
				});
				
				
				$('th').tipsy({

					// arrow width
					arrowWidth: 20, //arrow css border-width * 2, default is 5 * 2

					// default attributes for tipsy
					// data-tipsy-position | data-tipsy-offset | data-tipsy-disabled
					attr: 'data-tipsy', 

					// custom class
					cls: null, 

					// fadeIn, fadeOut animation duration
					duration: 150, 

					// offset from element
					offset: -7,

					// top-left | top-center | top-right | bottom-left
					// bottom-center | bottom-right | left | right
					position: 'top-center', 

					// hover | focus | click | manual
					trigger: 'hover',

					// events
					onShow: null,
					onHide: null 

				})
			}
		
		//setTimeout(function() {}, (300));
	});


	$("#rechercher").on("keyup touchend click", function() {

		var value = $(this).val().toLowerCase();
		
		$("#table_json table tbody tr").filter(function() {
			$(this).toggle($(this).text().toLowerCase().indexOf(value) > -1);
		});
	
		$('table').trigger('applyWidgets'); // BL indispensable pour que l'alternance des couleurs de lignes soit maintenue pendant la recherche (widget "Zebra" tablesorter.js cf. ci-dessous)
	});

	

	masquage_observations = 0;
	
	if (masquage_observations == 1) {
	
		$(".observation").each(function() {
			contenu = $(this).text();
			if (contenu != '' && contenu.indexOf('RAS') == -1) {
				$(this).html('<span class="tooltip">Consulter<span class="tooltiptext">'+contenu+'</span></span>');
			} else {
				$(this).addClass('neant');
			}
		});
	}
	

}); // fin document ready



$( function() {

	// 20221018 gestion des publications plus anciennes - suite
	// 20230116 : nouvelle version
	
	$("#annee_exercice").on("change", function() {

		var value = $(this).val();
		
		if (value != undefined && value !="") {
		
			var url_origine = window.location.href;
			
			if(url_origine.indexOf(value) == -1) {
			
				window.location.href=url_origine.replace(/[0-9]{4}/g, value);
			
			}
		
		}
	});
	
	// 20221018 gestion des publications plus anciennes - fin
} );

  